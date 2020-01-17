use svgtypes::{PathCommand, PathSegment};

use super::math::*;
use super::tick_timer::TickTimer;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub enum LineTo {
    Fly(Point),
    Draw(Point),
    Erase(Point),
}

impl LineTo {

    fn new(point: Point, move_type: MoveType) -> Self {
        match move_type {
            MoveType::Fly => LineTo::Fly(point),
            MoveType::Draw => LineTo::Draw(point),
            MoveType::Erase => LineTo::Erase(point),
        }
    }
}

pub fn points_from_path_segments(path_segments: impl Iterator<Item=PathSegment> + 'static) -> Box<dyn Iterator<Item = LineTo>> {
    let mut current_point = Point {x: 0., y: 0.};
    let mut prev_support_point_opt: Option<SupportPoint> = None;

    Box::new(path_segments.flat_map(move |path_segment| {
        let point_iterator =
            calc_point_iterator(&current_point, path_segment, &prev_support_point_opt);
        prev_support_point_opt = point_iterator.get_support_point();
        current_point = point_iterator.get_end_position();

        let move_type = point_iterator.move_type();
        point_iterator.map(move |point| LineTo::new(point, move_type)).into_iter()

    }).into_iter())
}

// === private members ===

#[derive(PartialEq, Copy, Clone)]
enum MoveType {
    Fly,
    Draw,
    Erase,
}

#[derive(Debug)]
struct SupportPoint {
    path_command: PathCommand,
    point: Point,
}

trait PointIterator: Iterator<Item=Point> {
    fn get_support_point(&self) -> Option<SupportPoint>; //support point is always in absolute
    fn get_end_position(&self) -> Point;
    fn move_type(&self) -> MoveType;
}

struct EmptyPointIterator {
    end_x: f64,
    end_y: f64,
}

impl Iterator for EmptyPointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl PointIterator for EmptyPointIterator {
    fn get_support_point(&self) -> Option<SupportPoint> {
        None
    }

    fn get_end_position(&self) -> Point {
        Point {
            x: self.end_x,
            y: self.end_y,
        }
    }

    fn move_type(&self) -> MoveType {
        MoveType::Fly
    }
}

struct LinePointIterator {
    end_x: f64,
    end_y: f64,
    move_type: MoveType,
    done: bool,
}

impl LinePointIterator {
    fn new(end_x: f64, end_y: f64, move_type: MoveType) -> Self {
        LinePointIterator {
            end_x,
            end_y,
            move_type,
            done: false,
        }
    }
}

impl Iterator for LinePointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                None
            } else {
                self.done = true;
                Some(Point {
                    x: self.end_x,
                    y: self.end_y,
                })
            }
    }
}

impl PointIterator for LinePointIterator {
    fn get_support_point(&self) -> Option<SupportPoint> {
        None
    }

    fn get_end_position(&self) -> Point {
        Point {
            x: self.end_x,
            y: self.end_y,
        }
    }

    fn move_type(&self) -> MoveType {
        self.move_type
    }
}

struct CurvePointIterator<F: Fn(f64) -> Point> {
    time: TickTimer,
    calc_formula: F,
    move_type: MoveType,
    support_point: Option<SupportPoint>,
}

impl<F: Fn(f64) -> Point> Iterator for CurvePointIterator<F> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

impl<F: Fn(f64) -> Point> PointIterator for CurvePointIterator<F> {
    fn get_support_point(&self) -> Option<SupportPoint> {
        match &self.support_point {
            Some(supp_p) => Some(SupportPoint {
                path_command: supp_p.path_command,
                point: Point {
                    x: supp_p.point.x,
                    y: supp_p.point.y,
                },
            }),
            None => None,
        }
    }

    fn get_end_position(&self) -> Point {
        (self.calc_formula)(1.0)
    }

    fn move_type(&self) -> MoveType {
        self.move_type
    }
}

struct EllipsePointIterator<F: Fn(f64) -> Point> {
    time: TickTimer,
    calc_formula: F,
    end_x: f64,
    end_y: f64,
}

impl<F: Fn(f64) -> Point> Iterator for EllipsePointIterator<F> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

impl<F: Fn(f64) -> Point> PointIterator for EllipsePointIterator<F> {
    fn get_support_point(&self) -> Option<SupportPoint> {
        None
    }

    fn get_end_position(&self) -> Point {
        Point {
            x: self.end_x,
            y: self.end_y,
        }
    }

    fn move_type(&self) -> MoveType {
        MoveType::Draw
    }
}

fn calc_point_iterator(
    current: &Point,
    next_segment: PathSegment,
    prev_support_point_opt: &Option<SupportPoint>,
) -> Box<dyn PointIterator> {
    match next_segment {
        PathSegment::MoveTo { abs, x, y } => Box::new(move_to(current, abs, x, y)),
        PathSegment::LineTo { abs, x, y } => Box::new(line_to(current, abs, x, y)),
        PathSegment::HorizontalLineTo { abs, x } => Box::new(line_to(current, abs, x, current.y)),
        PathSegment::VerticalLineTo { abs, y } => Box::new(line_to(current, abs, current.x, y)),
        PathSegment::CurveTo {
            abs,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => cubic_curve_to(current, abs, x1, y1, x2, y2, x, y, next_segment),
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => smooth_cubic_curve_to(
            current,
            abs,
            x2,
            y2,
            x,
            y,
            prev_support_point_opt,
            next_segment,
        ),
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            quadratic_curve_to(current, abs, x1, y1, x, y, next_segment)
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            smooth_quadratic_curve_to(current, abs, x, y, prev_support_point_opt, next_segment)
        }
        PathSegment::EllipticalArc {
            abs,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        } => ellipse_curve_to(
            current,
            abs,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        ),
        PathSegment::ClosePath { abs: _ } => {
            //todo: implement me
            return Box::new(EmptyPointIterator {
                end_x: 0.,
                end_y: 0.,
            });
        }
    }
}

fn move_to(current: &Point, abs: bool, x: f64, y: f64) -> LinePointIterator {
    let end_point = absolute_point_coord(&current, abs, x, y);
    LinePointIterator::new(end_point.x, end_point.y, MoveType::Fly)
}

fn line_to(current: &Point, abs: bool, x: f64, y: f64) -> LinePointIterator {
    let end_point = absolute_point_coord(&current, abs, x, y);
    LinePointIterator::new(end_point.x, end_point.y, MoveType::Draw)
}

fn cubic_curve_to(
    current: &Point,
    abs: bool,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x: f64,
    y: f64,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();
    let p1 = absolute_point_coord(&current, abs, x1, y1);
    let end_point = absolute_point_coord(&current, abs, x, y);
    let p2 = absolute_point_coord(&current, abs, x2, y2);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: Point { x: p2.x, y: p2.y },
    });
    let calc_formula = cubic_curve(current.x, current.y, p1, p2, end_point);
    Box::new(CurvePointIterator {
        time,
        calc_formula,
        move_type: MoveType::Draw,
        support_point,
    })
}

fn smooth_cubic_curve_to(
    current: &Point,
    abs: bool,
    x2: f64,
    y2: f64,
    x: f64,
    y: f64,
    prev_support_point_opt: &Option<SupportPoint>,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let p1 = mirrored_point(current, abs, prev_support_point_opt, CurveType::Cubic);
    cubic_curve_to(current, abs, p1.x, p1.y, x2, y2, x, y, next_segment)
}

fn quadratic_curve_to(
    current: &Point,
    abs: bool,
    x1: f64,
    y1: f64,
    x: f64,
    y: f64,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();
    let p1 = absolute_point_coord(&current, abs, x1, y1);
    let end_point = absolute_point_coord(&current, abs, x, y);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: Point { x: p1.x, y: p1.y },
    });
    let calc_formula = square_curve(current.x, current.y, p1, end_point);
    Box::new(CurvePointIterator {
        time,
        calc_formula,
        move_type: MoveType::Draw,
        support_point,
    })
}

fn smooth_quadratic_curve_to(
    current: &Point,
    abs: bool,
    x: f64,
    y: f64,
    prev_support_point_opt: &Option<SupportPoint>,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let p1 = mirrored_point(current, abs, prev_support_point_opt, CurveType::Quadratic);
    if p1.x == current.x && p1.y == current.y {
        Box::new(line_to(current, abs, x, y))
    } else {
        quadratic_curve_to(current, abs, p1.x, p1.y, x, y, next_segment)
    }
}

fn ellipse_curve_to(
    current: &Point,
    abs: bool,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
    large_arc: bool,
    sweep: bool,
    end_x: f64,
    end_y: f64,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();

    let end_point = absolute_point_coord(&current, abs, end_x, end_y);

    // If the endpoints are identical, then this is equivalent to omitting the elliptical arc segment entirely.
    if current.x == end_point.x && current.y == end_point.y {
        return Box::new(EmptyPointIterator {
            end_x: end_point.x,
            end_y: end_point.y,
        });
    }

    // If rx = 0 or ry = 0 then this arc is treated as a straight line segment joining the endpoints.
    if rx == 0. || ry == 0. {
        return Box::new(line_to(current, abs, end_x, end_y));
    }

    let (start_angle, sweep_angle, rx_abs, ry_abs, x_rad_rotation, center_x, center_y) =
        ellipse_support_calc(
            current,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            end_point.x,
            end_point.y,
        );

    let calc_formula = ellipse_curve(
        start_angle,
        sweep_angle,
        rx_abs,
        ry_abs,
        x_rad_rotation,
        center_x,
        center_y,
    );
    Box::new(EllipsePointIterator {
        time,
        calc_formula,
        end_x: end_point.x,
        end_y: end_point.y,
    })
}

fn absolute_point_coord(start: &Point, abs: bool, x: f64, y: f64) -> Point {
    match abs {
        true => Point { x, y },
        false => Point {
            x: x + start.x,
            y: y + start.y,
        },
    }
}

enum CurveType {
    Cubic,
    Quadratic,
}

fn path_command_condition(prev_support_point: &SupportPoint, curve_type: CurveType) -> bool {
    match curve_type {
        CurveType::Cubic => {
            prev_support_point.path_command == PathCommand::SmoothCurveTo
                || prev_support_point.path_command == PathCommand::CurveTo
        }

        CurveType::Quadratic => {
            prev_support_point.path_command == PathCommand::SmoothQuadratic
                || prev_support_point.path_command == PathCommand::Quadratic
        }
    }
}

fn mirrored_point(
    current: &Point,
    abs: bool,
    prev_support_point_opt: &Option<SupportPoint>,
    curve_type: CurveType,
) -> Point {
    let mut mirrored_point = match prev_support_point_opt {
        Some(ref prev_support_point) if path_command_condition(prev_support_point, curve_type) => {
            let mirrored_x = current.x - prev_support_point.point.x;
            let mirrored_y = current.y - prev_support_point.point.y;
            Point {
                x: mirrored_x,
                y: mirrored_y,
            }
        }
        _ => Point { x: 0., y: 0. },
    };

    if abs {
        mirrored_point.x += current.x;
        mirrored_point.y += current.y;
    }

    mirrored_point
}
