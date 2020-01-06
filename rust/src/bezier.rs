use svgtypes::{PathCommand, PathSegment};

#[derive(PartialEq)]
pub enum MoveType {
    Fly,
    Draw,
    Erase,
}

struct BezierTick {
    pub time: f64,
}

impl BezierTick {
    const TICK_PERIOD: f64 = 0.001; //todo: number of ticks should be calculated based on curve length

    fn new() -> BezierTick {
        BezierTick { time: 0.0 }
    }
}

impl Iterator for BezierTick {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.time > 1.0) {
            None
        } else {
            let current_value = self.time;
            self.time += BezierTick::TICK_PERIOD;
            Some(current_value)
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct SupportPoint {
    path_command: PathCommand,
    point: Point,
}

pub struct PointIterator {
    time: BezierTick,
    calc_formula: Box<dyn Fn(f64) -> Point>,
    pub move_type: MoveType,
    support_point: Option<SupportPoint>, //support point is always in absolute
}

impl PointIterator {
    pub fn get_support_point(&self) -> Option<SupportPoint> {
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

    pub fn get_end_position(&self) -> Point {
        (self.calc_formula)(1.0)
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

fn linear_curve(start_x: f64, start_y: f64, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let x = start_x * (1. - t) + end.x * t;
        let y = start_y * (1. - t) + end.y * t;
        Point { x, y }
    })
}

fn square_curve(start_x: f64, start_y: f64, p1: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let square_diff = diff * diff;
        let x = start_x * square_diff + p1.x * 2. * t * diff + end.x * square_t;
        let y = start_y * square_diff + p1.y * 2. * t * diff + end.y * square_t;
        Point { x, y }
    })
}

fn cubic_curve(
    start_x: f64,
    start_y: f64,
    p1: Point,
    p2: Point,
    end: Point,
) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let cube_t = square_t * t;
        let square_diff = diff * diff;
        let cube_diff = square_diff * diff;
        let x = start_x * cube_diff
            + p1.x * 3. * t * square_diff
            + p2.x * 3. * square_t * diff
            + end.x * cube_t;
        let y = start_y * cube_diff
            + p1.y * 3. * t * square_diff
            + p2.y * 3. * square_t * diff
            + end.y * cube_t;
        Point { x, y }
    })
}

const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn map_to_part_of_circle(time: f64) -> f64 {
    time / 1.0 * 2.0 * PI
}

fn ellipse_curve(
    start_x: f64,
    start_y: f64,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let x_rad_rotation: f64 = x_axis_rotation * PI / 180.0;
        let circle_time = map_to_part_of_circle(t);

        let x = rx * circle_time.cos();
        let y = ry * circle_time.sin();

        let x_after_rotation = start_x + x * x_rad_rotation.cos() - y * x_rad_rotation.sin();
        let y_after_rotation = start_y + x * x_rad_rotation.sin() + y * x_rad_rotation.cos();

        Point {
            x: x_after_rotation,
            y: y_after_rotation,
        }
    })
}

pub fn calc_point_iterator(
    current: Point,
    next_segment: PathSegment,
    prev_support_point_opt: Option<SupportPoint>,
) -> PointIterator {
    match next_segment {
        PathSegment::MoveTo { abs, x, y } => move_to(&current, abs, x, y),
        PathSegment::LineTo { abs, x, y } => line_to(&current, abs, x, y),
        PathSegment::HorizontalLineTo { abs, x } => line_to(&current, abs, x, current.y),
        PathSegment::VerticalLineTo { abs, y } => line_to(&current, abs, current.x, y),
        PathSegment::CurveTo {
            abs,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => cubic_curve_to(&current, abs, x1, y1, x2, y2, x, y, next_segment),
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => smooth_cubic_curve_to(
            &current,
            abs,
            x2,
            y2,
            x,
            y,
            prev_support_point_opt,
            next_segment,
        ),
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            quadratic_curve_to(&current, abs, x1, y1, x, y, next_segment)
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            smooth_quadratic_curve_to(&current, abs, x, y, prev_support_point_opt, next_segment)
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
        } => ellipse_curve_to(&current, abs, rx, ry, x_axis_rotation),
        //        PathSegment::ClosePath{abs} => ()
        _ => {
            //todo: remove me
            let time = BezierTick::new();
            let end_point = absolute_point_coord(&current, true, 20., 33.);
            let calc_formula = linear_curve(current.x, current.y, end_point);
            PointIterator {
                time,
                calc_formula,
                move_type: MoveType::Fly,
                support_point: None,
            }
        }
    }
}

fn line_point_iterator(
    current: &Point,
    abs: bool,
    x: f64,
    y: f64,
    move_type: MoveType,
) -> PointIterator {
    let time = BezierTick::new();
    let end_point = absolute_point_coord(&current, abs, x, y);
    let calc_formula = linear_curve(current.x, current.y, end_point);
    PointIterator {
        time,
        calc_formula,
        move_type,
        support_point: None,
    }
}

fn move_to(current: &Point, abs: bool, x: f64, y: f64) -> PointIterator {
    line_point_iterator(current, abs, x, y, MoveType::Fly)
}

fn line_to(current: &Point, abs: bool, x: f64, y: f64) -> PointIterator {
    line_point_iterator(current, abs, x, y, MoveType::Draw)
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
) -> PointIterator {
    let time = BezierTick::new();
    let p1 = absolute_point_coord(&current, abs, x1, y1);
    let end_point = absolute_point_coord(&current, abs, x, y);
    let p2 = absolute_point_coord(&current, abs, x2, y2);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: Point { x: p2.x, y: p2.y },
    });
    let calc_formula = cubic_curve(current.x, current.y, p1, p2, end_point);
    PointIterator {
        time,
        calc_formula,
        move_type: MoveType::Draw,
        support_point,
    }
}

fn smooth_cubic_curve_to(
    current: &Point,
    abs: bool,
    x2: f64,
    y2: f64,
    x: f64,
    y: f64,
    prev_support_point_opt: Option<SupportPoint>,
    next_segment: PathSegment,
) -> PointIterator {
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
) -> PointIterator {
    let time = BezierTick::new();
    let p1 = absolute_point_coord(&current, abs, x1, y1);
    let end_point = absolute_point_coord(&current, abs, x, y);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: Point { x: p1.x, y: p1.y },
    });
    let calc_formula = square_curve(current.x, current.y, p1, end_point);
    PointIterator {
        time,
        calc_formula,
        move_type: MoveType::Draw,
        support_point,
    }
}

fn smooth_quadratic_curve_to(
    current: &Point,
    abs: bool,
    x: f64,
    y: f64,
    prev_support_point_opt: Option<SupportPoint>,
    next_segment: PathSegment,
) -> PointIterator {
    let p1 = mirrored_point(current, abs, prev_support_point_opt, CurveType::Quadratic);
    quadratic_curve_to(current, abs, p1.x, p1.y, x, y, next_segment)
}

fn ellipse_curve_to(
    current: &Point,
    abs: bool,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
) -> PointIterator {
    let time = BezierTick::new();
    let calc_formula = ellipse_curve(current.x, current.y, rx, ry, x_axis_rotation);
    PointIterator {
        time,
        calc_formula,
        move_type: MoveType::Draw,
        support_point: None,
    }
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
    prev_support_point_opt: Option<SupportPoint>,
    curve_type: CurveType,
) -> Point {
    let mut mirrored_point = match prev_support_point_opt {
        Some(prev_support_point) if path_command_condition(&prev_support_point, curve_type) => {
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
