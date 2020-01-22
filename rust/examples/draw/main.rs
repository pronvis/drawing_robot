#![allow(unused_parens)]

use skulpin::AppControl;
use skulpin::CoordinateSystemHelper;
use skulpin::InputState;
use skulpin::LogicalSize;
use skulpin::TimeState;
use skulpin::VirtualKeyCode;
use skulpin::{AppHandler, CoordinateSystem};
use std::ffi::CString;

use drawing_robot::svg::svg_curve::{points_from_path_segments, LineTo, Point};
use std::collections::LinkedList;

fn points_to_draw() -> Box<dyn Iterator<Item = LineTo>> {
    let svg_string = "M198.901,545.277c-4.035-3.746-7.869-7.492-11.702-11.632c-4.641-5.126-7.264-11.435-9.08-17.941
						c-1.614-5.521-3.026-11.041-3.026-16.956c0-1.577,0-3.352-0.202-4.929c-2.421-11.436-1.009-23.068-1.21-34.503
						c1.614-6.506,1.413-13.407,2.018-20.11c2.018-7.492,1.211-15.378,3.43-22.87c3.43-23.462,8.676-43.375,15.334-65.063
						l1.21-3.943c0.202-0.591-0.807-2.366-1.413-2.563c-4.035-0.986-7.869-2.563-11.904-2.76c-4.237,0-8.474-0.395-12.509,0
						c-5.448,0.591-10.896,1.972-15.939,4.337c-2.623,1.183-4.439,3.155-4.641,6.112c0,1.38-2.421,1.774-3.43,0.986
						c-1.816-1.577-3.43-3.352-4.641-5.521c-0.404-0.789-0.605-1.971-0.404-2.76c0.605-2.366,1.614-4.338,3.229-6.31
						c4.842-5.323,10.896-8.28,17.755-10.055c6.658-1.774,13.317-2.958,20.378-2.366c1.816,0.197,3.43,0.395,5.246,0.789
						l12.106,2.366c0.605,0.197,2.22-0.592,2.421-1.38c1.816-4.14,3.43-8.083,5.044-12.224c3.229-7.886,7.062-15.378,11.097-22.871
						c5.246-10.449,11.097-20.307,17.755-29.771c3.43-4.731,7.264-8.872,10.896-13.407c2.22-2.76,5.246-4.535,8.272-6.112
						c2.825-1.577,6.053-1.38,9.08-0.395c3.632,1.183,6.658,3.746,8.071,7.295c1.614,4.141,2.623,8.478,1.412,13.013
						c-0.403,1.183-0.605,2.563-0.807,3.943c-1.21,12.815-4.641,25.04-8.07,37.46c-3.229,11.435-6.658,22.673-10.896,33.911
						c-0.403,1.183-0.605,2.563-1.009,3.746c-0.202,0.789,0.605,2.366,1.412,2.563c0.807,0.395,1.816,0.986,2.825,0.986
						c5.852,0.592,11.703,0.592,17.554,1.774c1.009,0.197,2.018,0.197,3.027,0c13.518-0.986,26.835-2.366,39.344-7.886
						c4.439-1.972,8.676-4.14,13.115-6.309c4.641-2.169,8.676-5.521,12.509-8.872c32.081-27.011,2.22-75.315-23.203-51.656
						c-0.605,1.38-2.421,0.986-3.632,0c-0.605-0.394-1.009-0.789-1.412-1.38c-1.009-1.38-2.018-2.76-2.825-4.141
						c21.589-27.602,78.689,7.886,55.083,63.485c-3.43,6.703-7.465,12.815-13.72,17.547c-4.237,2.958-8.07,6.309-12.509,9.069
						c-8.878,5.323-18.563,8.675-28.651,11.436c-0.202,0.197-0.605,0.394-1.009,0.394c-7.465,0.395-14.931,2.169-22.598,2.169
						c-1.614,0-3.43-0.197-5.044-0.395c-1.413-0.197-2.825-0.789-4.237-0.789c-10.29-0.197-20.378-2.958-30.467-5.323l-1.816-0.591
						c-0.807-0.197-2.421,0.591-2.825,1.38c-1.009,2.366-2.421,4.732-3.43,7.295c-7.264,17.153-16.141,33.517-25.019,50.078
						c-1.614,2.958-3.43,5.718-5.246,8.675c-1.21,1.972-2.018,4.14-2.018,6.703c-0.202,3.549-0.403,7.295-1.412,11.041
						c-0.404,1.577-0.404,3.352-0.404,4.929c-0.807,17.941-0.605,35.883,0,53.824c0,2.563,0.202,5.323,0.807,8.084
						c0.807,3.154,1.009,6.506,1.413,9.858c0.202,0.591,0.807,1.183,1.21,1.577c0.403,0.592,2.421,0,2.825-0.591
						c0.807-0.789,1.413-1.38,2.018-2.366l5.852-8.281c1.412-2.366,2.825-4.732,4.438-6.901c0.605-0.591,1.413-0.985,2.018-1.577
						c0.403-0.986,0.403-1.774,0.807-2.76c1.614-2.958,3.026-6.112,4.842-8.872c5.044-7.689,9.08-15.97,13.115-24.053
						c1.614-3.155,3.026-6.112,4.641-9.069c1.009-1.774,3.632-2.169,5.448-0.986c1.009,0.591,1.614,1.183,2.421,1.972
						c0.807,0.789,1.412,1.774,1.009,2.76c-3.229,7.492-6.255,14.984-9.685,22.279c-6.255,13.801-11.904,27.997-19.571,41.206
						c-0.807,1.577-1.21,2.958-2.623,3.943c-1.614,5.323-4.641,9.661-8.676,13.604c-2.825,2.563-6.053,3.154-9.685,1.972
						C205.358,549.22,201.928,547.643,198.901,545.277z M214.236,531.279c-0.202,0-0.403,0.197-0.403,0.197
						c-0.202,0-0.202,0.394-0.202,0.591L214.236,531.279z M215.446,400.563c3.43-4.337,5.246-9.463,7.869-14.195
						c3.833-7.492,7.062-15.181,10.492-22.871c0.807-1.774,0.404-2.76-1.009-3.352l-7.465-3.549
						c-1.614-0.592-3.833,0.394-4.237,1.972l-4.842,23.659c-0.807,4.338-1.614,8.478-2.018,11.041
						c-0.202,3.549-0.403,5.126-0.403,6.704c0,0.197,0.403,0.591,0.605,0.591C214.841,400.76,215.244,400.76,215.446,400.563z
						 M214.639,530.885c1.009-0.197,1.816-0.789,1.816-1.774C215.446,529.11,214.841,529.899,214.639,530.885L214.639,530.885z
						 M220.692,536.997v-3.746c-2.421,0.591-3.632,2.366-5.044,4.14c-0.202,0.197-0.202,0.592,0,0.789
						c0.403,0.789,1.009,0.986,1.816,0.592C218.675,538.179,219.482,537.588,220.692,536.997z M220.692,522.998
						c-3.026,0.986-4.035,3.155-4.035,5.915C219.683,527.927,220.692,525.759,220.692,522.998z M220.49,541.531
						c-0.404,0-0.807,0.197-1.21,0.394l0.605,0.592C220.087,542.32,220.289,541.728,220.49,541.531z M224.727,517.281
						c-2.825,0.789-4.035,2.563-3.833,5.52C223.517,522.012,224.526,519.844,224.727,517.281z M221.499,532.659
						c0.202-0.197,0.202-0.394,0.202-0.591c-0.605,0-0.807,0.197-0.807,0.789C221.095,532.856,221.298,532.856,221.499,532.659z
						 M222.71,538.771v-1.578h-1.816v3.943C221.903,540.742,222.71,539.954,222.71,538.771z M224.727,535.222
						c-0.807,0.394-1.412,0.986-1.614,1.774C223.92,536.602,224.526,536.011,224.727,535.222z M226.947,512.943
						c-1.816,0.592-2.018,1.972-1.816,3.549C226.543,515.703,226.947,514.521,226.947,512.943z M226.745,535.025
						c-0.807-1.38-0.807-1.38-1.614,0H226.745z M240.062,342.401c1.009-0.197,1.816-0.789,2.623-1.38
						c0.605-0.394,0.807-1.183,1.009-1.774c5.246-14.393,9.887-28.785,14.124-43.375c1.211-4.14,1.614-8.675,2.825-12.815
						c2.018-6.112,1.816-12.618,3.43-18.927c0.605-1.577,0.202-3.352,0.202-4.929c0-0.789-1.21-1.774-2.219-1.972h-1.614
						c-0.605,1.972-1.614,3.549-2.825,5.126c-5.851,8.675-10.896,17.941-15.132,27.603c-6.457,14.59-11.501,29.574-15.536,44.952
						c-0.807,2.563,0.403,4.535,3.228,5.521c2.22,0.591,4.439,1.38,6.86,1.972C238.044,342.598,239.053,342.598,240.062,342.401z
						 M228.965,508.803h-1.816v3.746C228.965,511.957,228.763,510.183,228.965,508.803z M229.772,508.212
						c0,0,0.202-0.197,0.202-0.395l-0.403,0.591L229.772,508.212z M241.676,263.143c0-0.197,0.202-0.394,0.403-0.394
						c-0.403,0-0.403,0-0.605,0.197c-0.202,0.197-0.404,0.394-0.404,0.591L241.676,263.143z M242.483,262.157
						c0,0-0.202,0-0.202,0.197L242.483,262.157z M245.51,257.228c-0.202,0-0.403,0-0.605,0.197c-0.202,0-0.202,0.394-0.202,0.591
						C245.308,258.017,245.51,257.82,245.51,257.228z M247.729,255.06c-1.21,0-1.816,0.789-1.816,1.774
						C246.922,256.637,247.527,256.045,247.729,255.06z M252.773,250.525c0.202-0.197,0.202-0.591,0-0.789
						c-0.404-0.591-1.009-0.591-1.614-0.197c-2.219,1.183-3.632,2.76-3.228,5.323C250.15,254.271,251.563,252.496,252.773,250.525z
						 M255.598,253.285h-0.202c0,0.197-0.202,0.395-0.202,0.592L255.598,253.285z M260.037,252.694c0-0.591,0-1.183-0.202-1.577
						c-0.202-1.183-1.009-1.38-2.018-0.592c-0.807,0.592-1.412,1.183-1.614,2.169H260.037z M260.037,255.257
						c-1.009,0.789-1.009,0.789,0,1.38V255.257z M261.449,253.877l-1.009-0.592v1.38
						C260.844,254.271,261.046,254.271,261.449,253.877z M323.795,353.245c0-0.197,0.202-0.197,0.202-0.395
						C323.795,352.85,323.795,353.048,323.795,353.245L323.795,353.245z M329.646,352.062c6.457-2.563,12.308-5.915,17.352-10.646
						c0.605-0.395,0.807-0.986,1.21-1.578c0.202-0.197,0-0.591-0.202-0.789s-0.605-0.394-0.807-0.394
						c-7.465,3.943-14.527,8.675-21.993,13.012c-0.202,0-0.202,0.395-0.403,0.592C326.62,352.456,328.234,352.653,329.646,352.062z
						 M330.453,340.429l-1.009,0.591l0.605,0.395C330.252,341.218,330.252,340.823,330.453,340.429z M334.892,338.26
						c-1.614-0.197-3.229,0-3.833,1.774c0.605,0,1.009,0.197,1.412,0.197C333.884,340.232,334.691,339.444,334.892,338.26z
						 M336.91,354.625h-1.614c-0.202,0-0.605,0.197-0.807,0.395c-0.202,0-0.403,0.394-0.202,0.591
						c0.404,0.394,1.009,0.591,1.614,0.197L336.91,354.625z M337.919,337.866c2.421-1.578,4.641-3.352,6.86-5.126
						c0.605-0.591,0.403-1.577,0.605-2.563c-4.237,1.183-7.667,3.746-10.088,7.098c-0.202,0.197-0.202,0.395-0.202,0.789
						C336.103,338.063,337.112,338.458,337.919,337.866z M338.927,354.23c3.833-1.38,7.465-3.154,10.088-6.112
						c0.404-0.591,0.404-1.183,0.404-1.774c-0.605,0-1.211-0.394-1.816,0c-3.632,1.577-6.86,3.746-9.887,6.506
						c-0.404,0.197-0.202,0.789-0.404,1.38C337.919,354.23,338.524,354.427,338.927,354.23z M347.402,328.205
						c-1.009,0-1.816,0.592-2.018,1.774C346.594,329.98,347.2,329.388,347.402,328.205z M348.209,327.614l0.202-0.395l-0.404,0.395
						H348.209z M349.621,346.147c1.211,0,2.018-0.591,1.816-1.774C350.428,344.57,349.621,344.964,349.621,346.147L349.621,346.147z
						 M352.446,343.781c0.202,0,0.202-0.197,0.202-0.591c-0.605,0-0.807,0.197-0.807,0.789
						C352.042,343.978,352.244,343.978,352.446,343.781z M283.844,542.32c-5.044-2.169-10.088-4.337-14.527-7.689c-5.851-4.337-10.896-9.463-15.334-14.984
						c-4.842-6.506-7.667-13.604-8.272-21.49c0-1.38,0-2.76-0.202-4.141c-1.009-13.998,2.22-27.405,7.264-40.417l1.816-4.732
						c4.035-10.055,10.492-18.335,17.958-25.828c5.246-5.126,11.5-8.675,18.563-10.646c4.438-1.38,8.877-1.183,13.115,1.183
						c1.412,0.985,3.026,1.774,4.439,2.563c2.623,1.774,4.641,4.14,5.851,6.9c1.614,3.746,2.421,7.689,3.026,11.83
						c0.202,0.986,0,1.972-0.202,2.958c-1.614,7.295-4.439,14.195-10.088,19.519c-4.237,4.14-8.676,7.886-13.922,10.646
						c-6.658,3.352-13.115,7.295-20.177,10.055c-1.816,0.591-2.825,2.169-2.825,3.943c0,7.689-0.403,15.575,0,23.265
						c0.404,6.112,2.421,11.83,5.649,17.153c2.018,2.958,5.044,4.535,8.676,4.732c1.816,0,3.632-0.197,5.246-0.592
						c12.308-3.352,22.396-10.252,30.467-19.716c9.483-10.844,17.15-22.87,25.02-34.7c0.807-1.183,1.412-2.563,2.623-3.155
						c0.605-0.197,1.413-0.394,2.018-0.394c2.825,0.789,4.237,3.746,3.026,6.309c-0.605,1.38-1.211,2.563-2.018,3.746
						c-5.246,9.069-10.492,18.139-15.94,27.011c-3.43,5.521-7.465,10.647-11.097,15.97c-0.403,0.394-1.21,0.789-1.614,1.183
						c-0.605-0.985-1.211-1.577-2.018-1.971v1.971c0.807,0,1.211,0.197,2.018,0.197c-0.403,0.789-1.21,1.577-2.018,1.774
						c0-0.591-0.202-1.183-0.202-1.774h-1.816c-0.202,0.789-0.202,1.183-0.202,1.774c-1.009,0.395-1.614,1.183-2.219,2.366
						c-1.009,0.789-0.807,0.985,0.403,1.971l-0.403-0.197c-0.404,0.789-1.009,1.774-1.614,2.563
						c-2.22,2.563-5.246,4.337-7.869,6.309c-5.851,4.337-12.308,5.915-19.571,5.126C285.862,542.714,284.853,542.714,283.844,542.32
						z M267.703,436.051c-0.202,0-0.202,0.197-0.403,0.394L267.703,436.051z M270.326,431.714c-1.816,0.591-1.816,2.168-1.816,3.746
						C269.923,434.671,270.528,433.291,270.326,431.714z M272.344,429.545c-1.009,0-1.614,0.789-1.816,1.774
						C271.739,431.517,272.344,430.728,272.344,429.545z M272.545,429.348c1.211,0,1.816-0.591,2.018-1.774
						C273.151,427.573,272.545,428.165,272.545,429.348L272.545,429.348z M277.993,464.442l11.501-7.098
						c4.035-2.366,7.667-5.521,11.097-8.872c6.255-6.309,8.676-13.604,7.465-22.279c-0.202-0.986-0.605-1.972-1.009-2.76
						c-0.605-1.38-3.026-2.563-4.439-2.169c-1.009,0.197-2.018,0.395-2.825,0.789c-3.43,1.183-6.053,3.352-8.676,5.521l0.202-0.197
						h-0.202c0-1.577-0.202-2.957-1.816-3.746v-0.394c-0.202,0-0.202,0.197-0.404,0.197c-0.202-0.395-0.605-1.183-1.009-1.38
						c-0.403-0.197-1.412-0.197-1.816,0c-2.219,1.183-3.43,2.76-3.026,5.323c1.21-0.789,2.623-1.38,3.833-1.972v0.197h0.202
						c0.202,1.38,0.202,3.155,2.018,3.943v0.197l0.202-0.197c-0.404,0.394-0.807,0.986-1.211,1.577
						c-6.053,8.083-10.088,16.759-13.115,26.222c-0.403,1.38-0.605,2.563-0.807,3.943c-0.202,1.774,0.404,3.746,1.009,3.746
						C276.177,465.033,277.186,464.837,277.993,464.442z M276.581,425.602c-1.21,0-2.018,0.592-1.816,1.774
						C275.774,427.376,276.379,426.588,276.581,425.602z M278.598,423.63c-1.009,0-1.816,0.592-1.816,1.774
						C277.791,425.208,278.397,424.616,278.598,423.63z M279.607,423.039c0-0.197,0-0.394,0.202-0.591
						c-0.605-0.197-0.807,0-1.009,0.789C279.002,423.236,279.204,423.236,279.607,423.039z M282.432,427.771h-0.202l-0.403,0.591
						L282.432,427.771z M287.073,425.405h-0.202c0.403-0.985,1.009-1.577,2.018-1.971c0.202,0,0.202,0,0.404,0.197
						c-0.404,0.986-1.009,1.774-2.22,1.972V425.405z M291.108,427.376v0.197c-0.605,0.591-1.413,1.38-1.816,1.972h-0.202
						C289.494,428.559,290.099,427.771,291.108,427.376z M311.487,525.167l-0.202,0.197l-0.404,0.591
						C311.083,525.759,311.285,525.561,311.487,525.167z M312.293,524.575c0.807-0.197,1.413-0.591,1.614-1.577
						C313.1,523.196,312.495,523.787,312.293,524.575L312.293,524.575z M317.943,522.801c0.403-0.985,1.21-1.774,2.219-2.168v-3.549
						c-2.623,1.183-4.842,2.957-6.053,5.717H317.943z M316.329,527.139h1.614c-0.202,0.789-0.807,1.38-1.614,1.774V527.139z
						 M318.346,524.97c0.403,0,1.009,0.197,1.816,0.197c-0.403,0.789-1.009,1.38-1.816,1.577V524.97z M320.969,516.492
						c0-0.197,0.202-0.394,0.202-0.591l-0.403,0.591H320.969z M407.527,514.323c-1.21-3.746-2.018-7.886-2.421-11.83c-0.807-9.858-0.404-19.913,0-29.771
						c0.202-2.366,0-4.929-0.202-7.295c0-0.395-0.202-0.789-0.404-0.789c-0.605-0.197-1.412-0.197-1.816,0
						c-1.21,1.183-2.219,2.366-3.026,3.549c-3.229,3.943-6.053,8.083-9.281,11.83c-3.229,3.746-6.658,7.492-10.29,10.844
						c-2.421,2.168-5.044,4.14-7.667,5.521c-13.922,7.886-28.853,1.38-32.484-13.801c-1.009-3.943-1.009-7.886-0.403-12.027
						c1.412-8.675,3.833-16.956,8.474-24.447c0.807-1.183,1.816-2.169,2.825-3.352c5.649-10.055,13.922-17.547,24.414-22.87
						c9.483-4.732,19.369-4.929,29.458-0.986c1.614,0.789,3.43,1.774,5.044,2.366c1.21,0.395,2.623-0.197,3.228-1.38
						c0.807-1.38,1.413-2.76,2.018-4.141l3.632-11.632c1.412-4.337,2.623-8.872,4.237-13.209c1.009-2.76,2.22-5.323,3.43-7.689
						c2.219-3.943,4.842-4.535,8.474-1.972c2.421,1.775,4.237,3.943,4.035,7.098c-0.202,4.929,0,9.858-0.605,14.787
						c-1.211,9.858-3.027,19.716-4.439,29.377c-1.009,7.689-2.018,15.378-2.825,23.067c-1.816,13.998-3.43,27.799-5.044,41.601
						c-0.807,6.506-1.412,13.012-1.816,19.519c-0.202,1.38,0.404,2.958,0.807,4.337c0.404,0.592,1.614,1.183,2.421,1.183
						s1.816-0.394,2.623-0.986c4.035-1.972,7.264-4.929,10.088-8.281c3.43-4.535,7.062-8.872,10.29-13.407
						c5.649-8.675,10.896-17.547,16.343-26.222c0.807-1.38,1.816-2.563,2.825-3.746c0.403-0.591,2.623-0.591,3.631,0
						c1.009,0.789,1.614,1.972,1.413,3.352c-0.202,1.183-0.807,2.366-1.412,3.352c-10.694,14.984-19.773,31.151-31.678,45.347
						c-1.816,2.168-3.632,4.337-5.649,6.309c-1.816,1.577-4.035,2.957-6.255,4.337c-2.018,1.183-4.237,0.986-6.254,0
						C415.598,528.322,410.15,522.407,407.527,514.323z M408.133,431.911c1.614-3.943-0.605-7.689-4.842-8.281
						c-4.641-0.789-9.483-0.789-14.124,0.592c-41.968,11.041-33.494,79.652-13.317,57.373
						C390.781,467.597,400.667,450.444,408.133,431.911z M361.121,438.614c0.605-0.592,1.211-1.183,1.816-1.972l2.018-1.774
						l-0.202-0.197l-2.018,1.774c-0.605,0.789-1.21,1.38-1.816,1.972l-0.202,0.395L361.121,438.614z M381.903,483.172
						c0.807-0.591,1.412-1.183,2.219-1.577l1.614-2.168c0.807-0.592,1.412-1.183,2.018-1.775l-0.202-0.197
						c-0.605,0.591-1.21,1.183-2.018,1.774c-0.404,0.789-1.009,1.38-1.614,2.168c-0.807,0.395-1.413,0.986-2.22,1.578v0.394
						L381.903,483.172z M429.722,413.378c0.605-0.592,1.009-1.577,1.412-2.366c2.219-3.746,3.43-7.886,3.833-12.224
						c0.605-3.352,1.413-6.704-0.403-10.844c-0.807,1.38-1.211,1.774-1.413,2.366c-1.009,7.492-3.833,14.787-6.053,22.082
						c0,0.591,0.202,1.183,0.403,1.774C428.108,413.772,429.116,413.772,429.722,413.378z M485.209,525.561c-7.869-1.577-12.913-6.309-16.343-13.012c-2.623-5.126-4.439-10.647-5.649-16.167
						c-2.22-9.267-2.018-9.661-2.421-20.11c-0.202-11.83,2.219-23.462,6.255-34.503c3.228-8.675,7.667-16.758,13.115-24.25
						c2.421-3.549,5.649-6.703,9.281-9.463c21.186-16.364,37.125,0.197,27.037,32.137c0,1.774-0.807,3.352-1.816,4.732
						c-0.605,0.789-1.412,1.577-2.421,1.38c-1.009,0-2.22,0-3.229-0.197c-0.807-0.197-1.816-1.38-1.816-2.169
						c0.202-1.38,0-2.76,0.404-4.141c6.658-18.138,5.448-39.234-8.272-25.828c-1.413,1.38-3.229,2.76-4.237,4.337
						c-1.816,3.549-4.842,5.521-7.465,8.083l0.202-0.197h-2.018c-0.202-1.183-0.202-2.366-0.403-3.549
						c-1.816,0.591-1.816,2.168-2.018,3.549c-1.614,0.986-1.816,2.366-1.816,3.943c0.807,0.197,1.211,0.197,1.816,0.395
						c0.403,1.577,0.605,3.154,2.421,4.14l-0.403-0.197c-0.807,3.352-1.211,6.704-2.421,9.661
						c-2.219,5.915-1.614,12.224-3.43,17.941c0,6.506-0.807,12.815,0.807,19.125c0.605,2.366,0.404,4.731,0.807,7.098
						c1.614,8.281,3.43,16.561,7.869,24.053l2.219,3.352c1.614,2.169,5.044,3.155,7.264,1.578c3.228-1.972,6.457-3.943,9.483-6.112
						c4.842-3.943,9.08-8.478,12.913-13.209c5.649-7.689,11.097-15.576,15.334-24.053c0.403-0.789,1.009-1.774,1.614-2.563
						c1.21-2.169,3.631-2.76,6.254-1.972c0.605,0.394,1.614,1.972,1.413,2.563s-0.202,1.38-0.605,1.972
						c-4.842,10.055-9.886,20.11-16.141,29.376c-4.641,7.098-10.088,13.604-17.15,18.73c-7.062,4.929-14.325,6.506-22.598,4.535
						C487.832,526.153,486.42,525.759,485.209,525.561z M472.901,440.389c0.202-1.183,0-2.366,0.202-3.746
						c-0.403,0.197-0.605,0.197-0.807,0.395c-3.026,4.14-4.237,8.872-5.246,13.801c-0.202,0.986,0.403,1.972,0.807,3.549
						L472.901,440.389z M475.121,432.503c-1.816,0.591-1.816,2.168-1.816,3.943C475.121,435.854,475.121,434.277,475.121,432.503z
						 M477.138,428.559c-1.009,0.395-1.413,1.183-1.614,1.972c-0.202,0.394,0,1.183-0.202,1.577
						C476.936,431.517,477.138,430.136,477.138,428.559z M478.752,426.982c0.404-0.789,0-1.38-0.605-1.183
						c-0.202,0-0.404,0.395-0.404,0.789c-0.202,0.395-0.202,0.986-0.202,1.577L478.752,426.982z M481.174,432.108v-1.38
						l-1.009,0.789c-0.202,0.197-0.202,0.591-0.202,0.789c0,0.394,0.202,0.789,0.403,0.789
						C480.77,433.291,481.174,432.897,481.174,432.108z M483.595,426.39h1.816v3.943h-1.816V426.39z M486.42,421.856
						c0,0,0-0.394,0.202-0.591c-0.403,0-0.807,0.197-0.807,0.789C486.016,422.053,486.218,422.053,486.42,421.856z M485.814,430.531
						c1.009,0.394,1.413,0.986,1.009,1.774c-0.202,0.591-0.605,1.183-1.009,1.577V430.531z M488.034,427.179
						c-0.202-0.394-0.202-0.591-0.202-0.789c0.605,0,1.009,0.395,0.807,0.986C488.437,427.376,488.034,427.376,488.034,427.179z M544.125,517.084c-0.403-0.789-0.605-1.578-0.807-2.563c-3.026-13.999-7.062-27.603-9.281-41.798
						c-0.403-2.366-0.605-4.929-0.605-7.295c0-11.238-0.404-22.476,0.202-33.517c0.807-10.646,2.219-21.096,4.237-31.348
						c1.816-10.646,4.237-21.293,6.86-31.94c5.851-25.236,16.142-48.895,28.853-71.568c5.044-8.675,10.896-16.759,17.554-24.25
						c4.237-4.535,9.08-8.675,14.729-11.632c6.255-3.352,12.106-2.563,17.554,1.971c5.246,4.141,7.869,9.464,8.272,15.97
						c0.404,9.858-0.807,19.321-3.026,28.588c-3.43,13.999-8.878,27.208-15.132,40.22c-18.159,35.686-32.888,57.965-51.854,83.99
						l-0.605,0.789c-4.641,5.323-6.659,11.436-6.255,18.533c0.403,5.915,0.202,11.632,0.202,17.547c0,1.38,0.404,2.76,0.605,4.535
						c3.833-0.789,3.632-4.14,5.448-5.717c0.807-1.972,1.614-3.746,2.219-5.521c1.413-3.155,2.825-6.309,4.641-9.267
						c1.21-2.169,2.825-4.14,4.439-5.915c1.816-1.774,4.439-2.958,6.86-2.563c7.466,1.183,14.124,3.549,18.966,9.661
						c2.825,3.352,5.044,7.098,6.457,11.238c2.219,5.915,3.632,11.83,4.439,17.941c0.403,4.338,1.009,8.675,1.614,13.013
						c0.202,0.986,2.018,1.972,3.026,1.38c2.421-1.577,5.044-2.76,7.062-4.731c4.237-4.141,8.071-8.675,12.106-13.013
						c3.228-3.549,6.254-7.098,9.281-10.646c1.009-1.183,2.825-1.38,4.237-0.395c0.807,0.395,1.211,1.578,1.211,2.366
						s-0.404,1.774-1.009,2.563c-4.035,5.718-8.272,11.632-12.509,17.35c-3.43,4.14-7.667,7.492-12.106,10.449
						c-5.852,3.746-11.904,3.943-18.159,1.183c-11.299-4.929-17.957-13.21-20.782-24.842c-1.009-4.337-1.614-8.675-2.421-12.815
						c-0.404-2.168-1.614-3.352-3.833-3.352l0.202,0.197c-6.053,0.197-7.465,10.844-8.676,13.604
						c-5.852,13.604-8.272,27.997-10.492,42.389c-0.202,1.38,0.202,2.958,0,4.535c-0.202,0.591-1.21,1.577-1.816,1.774
						C549.774,524.773,546.143,522.604,544.125,517.084z M547.353,427.376c0.403-0.789,0-1.183-0.404-1.972
						c-0.403,0.592-0.605,1.38-1.009,2.169c-0.202,0.197,0,0.789,0,0.789C546.949,428.559,547.152,428.165,547.353,427.376z
						 M547.959,423.236c1.412-4.732,2.825-9.464,2.219-14.393c0-0.395-0.202-0.591-0.404-0.789
						c-0.202,0.394-0.202,0.591-0.403,0.591c-0.403,1.38-1.614,2.76-1.412,3.943c-0.202,3.943-2.623,7.492-1.614,11.83
						c0.202,0.197,0.403,0.394,0.605,0.394C547.353,424.419,547.757,424.025,547.959,423.236z M551.792,404.9
						c0.403-0.986,0.202-1.972-0.807-2.958c-0.404,0.789-0.807,1.578-1.413,2.366c-0.605,1.183-0.807,2.168,0.202,3.352
						C550.38,406.872,551.187,405.886,551.792,404.9z M552.801,398.591c0.605-0.986,0.403-2.168-0.202-2.957
						c-0.605,0.789-1.211,1.577-1.816,2.957c-0.605,0.789-0.202,1.774,0.202,2.76C551.59,400.563,551.994,399.38,552.801,398.591z
						 M553.81,392.874c0.605-1.183,0.807-2.366,0.202-3.549c-0.605,0.986-1.21,1.775-1.816,2.958
						c-0.807,0.986-0.403,1.972,0.202,2.958L553.81,392.874z M555.222,386.565c0.404-0.986,0.807-2.366,0-3.549
						c-0.403,0.789-1.009,1.38-1.412,2.168c-0.605,1.38-1.009,2.563,0,3.746L555.222,386.565z M555.222,408.843
						c0.404-0.789,0-1.183-0.403-1.972c-0.404,0.591-0.605,1.38-1.009,2.169c-0.202,0.197,0,0.789,0,0.789
						C554.819,410.027,555.02,409.632,555.222,408.843z M555.827,404.703c1.413-4.731,2.825-9.463,2.219-14.393
						c0-0.394-0.202-0.591-0.403-0.789c-0.202,0.395-0.202,0.592-0.403,0.592c-0.404,1.38-1.614,2.76-1.413,3.943
						c-0.202,3.943-2.623,7.492-1.614,11.83c0.202,0.197,0.404,0.395,0.605,0.395C555.222,405.886,555.626,405.492,555.827,404.703z
						 M557.442,378.481c0.605-1.38,0.807-2.366,0-3.352c-1.009,1.183-1.614,2.563-2.22,3.746c-0.807,1.183-0.807,2.366-0.202,3.549
						C555.827,381.044,556.635,380.058,557.442,378.481z M559.056,371.186c0-0.197,0.202-0.789,0-0.986
						c-0.404,0-1.009,0.197-1.413,0.789c-0.807,0.986-1.009,2.366-0.202,3.352C557.845,373.355,558.45,372.172,559.056,371.186z
						 M559.661,386.367c0.404-0.986,0.202-1.972-0.807-2.958c-0.403,0.789-0.807,1.577-1.412,2.366
						c-0.605,1.183-0.807,2.169,0.202,3.352C558.249,388.339,559.056,387.353,559.661,386.367z M599.409,341.415
						c6.658-15.773,12.711-31.545,16.747-48.107c1.413-5.521,2.825-10.844,2.623-16.562c-0.202-2.168-1.816-2.957-3.833-3.154
						c-1.21-1.38-2.623-1.774-4.035-0.591c-0.605,0.591-1.211,1.183-1.614,1.971c1.21,0.197,2.623,0.197,3.833,0.395
						c-7.062,6.704-12.509,14.59-17.554,23.067c-12.308,20.899-21.993,43.178-28.449,66.64c-4.035,14.59-7.062,29.377-8.877,44.558
						c-0.404,2.366-0.605,4.337-1.211,11.041C582.057,384.001,591.54,358.962,599.409,341.415z M560.67,380.058
						c0.605-0.986,0.404-2.169-0.202-2.958c-0.605,0.789-1.21,1.577-1.816,2.958c-0.605,0.789-0.202,1.774,0.202,2.76
						C559.459,382.03,559.863,380.847,560.67,380.058z M561.678,374.341c0.605-1.183,0.807-2.366,0.202-3.549
						c-0.605,0.986-1.211,1.774-1.816,2.958c-0.807,0.986-0.404,1.972,0.202,2.957L561.678,374.341z M563.091,368.031
						c0.403-0.986,0.807-2.366,0-3.549c-0.404,0.789-1.009,1.38-1.413,2.169c-0.605,1.38-1.009,2.563,0,3.746L563.091,368.031z
						 M565.31,359.948c0.605-1.38,0.807-2.366,0-3.352c-1.009,1.183-1.614,2.563-2.219,3.746c-0.807,1.183-0.807,2.366-0.202,3.549
						C563.697,362.511,564.503,361.525,565.31,359.948z M566.925,352.653c0-0.197,0.202-0.789,0-0.986
						c-0.403,0-1.009,0.197-1.412,0.789c-0.807,0.986-1.009,2.366-0.202,3.352C565.714,354.822,566.319,353.639,566.925,352.653z
						 M605.26,266.889c0.202-0.197-0.202-0.986-0.404-1.577h-0.807c-5.044,2.958-9.08,6.704-11.299,12.027l-0.403,1.577
						C596.786,274.776,601.023,271.03,605.26,266.889z M609.094,269.846c-1.009,0-1.614,0.789-1.614,1.774
						C608.489,271.424,609.094,270.832,609.094,269.846z M610.91,269.649c0.202,0,0.202-0.394,0.403-0.591
						c-0.403-0.197-0.807-0.591-1.412-0.789l-0.605,1.38C609.901,269.649,610.304,269.846,610.91,269.649z M672.451,542.32c-5.044-2.169-10.088-4.337-14.527-7.689c-5.851-4.337-10.896-9.463-15.334-14.984
						c-4.842-6.506-7.667-13.604-8.272-21.49c0-1.38,0-2.76-0.202-4.141c-1.009-13.998,2.22-27.405,7.264-40.417l1.816-4.732
						c4.035-10.055,10.492-18.335,17.958-25.828c5.246-5.126,11.5-8.675,18.563-10.646c4.438-1.38,8.877-1.183,13.115,1.183
						c1.412,0.985,3.026,1.774,4.439,2.563c2.623,1.774,4.641,4.14,5.851,6.9c1.614,3.746,2.421,7.689,3.026,11.83
						c0.202,0.986,0,1.972-0.202,2.958c-1.614,7.295-4.439,14.195-10.088,19.519c-4.237,4.14-8.676,7.886-13.922,10.646
						c-6.658,3.352-13.115,7.295-20.177,10.055c-1.816,0.591-2.825,2.169-2.825,3.943c0,7.689-0.403,15.575,0,23.265
						c0.404,6.112,2.421,11.83,5.649,17.153c2.018,2.958,5.044,4.535,8.676,4.732c1.816,0,3.632-0.197,5.246-0.592
						c12.308-3.352,22.396-10.252,30.467-19.716c9.483-10.844,17.15-22.87,25.02-34.7c0.807-1.183,1.412-2.563,2.623-3.155
						c0.605-0.197,1.413-0.394,2.018-0.394c2.825,0.789,4.237,3.746,3.026,6.309c-0.605,1.38-1.211,2.563-2.018,3.746
						c-5.246,9.069-10.492,18.139-15.94,27.011c-3.43,5.521-7.465,10.647-11.097,15.97c-0.403,0.394-1.21,0.789-1.614,1.183
						c-0.605-0.985-1.211-1.577-2.018-1.971v1.971c0.807,0,1.211,0.197,2.018,0.197c-0.403,0.789-1.21,1.577-2.018,1.774
						c0-0.591-0.202-1.183-0.202-1.774h-1.816c-0.202,0.789-0.202,1.183-0.202,1.774c-1.009,0.395-1.614,1.183-2.219,2.366
						c-1.009,0.789-0.807,0.985,0.403,1.971l-0.403-0.197c-0.404,0.789-1.009,1.774-1.614,2.563
						c-2.22,2.563-5.246,4.337-7.869,6.309c-5.851,4.337-12.308,5.915-19.571,5.126C674.469,542.714,673.46,542.714,672.451,542.32z
						 M656.309,436.051c-0.202,0-0.202,0.197-0.403,0.394L656.309,436.051z M658.932,431.714c-1.816,0.591-1.816,2.168-1.816,3.746
						C658.529,434.671,659.134,433.291,658.932,431.714z M660.95,429.545c-1.009,0-1.614,0.789-1.816,1.774
						C660.345,431.517,660.95,430.728,660.95,429.545z M661.152,429.348c1.211,0,1.816-0.591,2.018-1.774
						C661.757,427.573,661.152,428.165,661.152,429.348L661.152,429.348z M666.599,464.442l11.501-7.098
						c4.035-2.366,7.667-5.521,11.097-8.872c6.255-6.309,8.676-13.604,7.465-22.279c-0.202-0.986-0.605-1.972-1.009-2.76
						c-0.605-1.38-3.026-2.563-4.439-2.169c-1.009,0.197-2.018,0.395-2.825,0.789c-3.43,1.183-6.053,3.352-8.676,5.521l0.202-0.197
						h-0.202c0-1.577-0.202-2.957-1.816-3.746v-0.394c-0.202,0-0.202,0.197-0.404,0.197c-0.202-0.395-0.605-1.183-1.009-1.38
						c-0.403-0.197-1.412-0.197-1.816,0c-2.219,1.183-3.43,2.76-3.026,5.323c1.21-0.789,2.623-1.38,3.833-1.972v0.197h0.202
						c0.202,1.38,0.202,3.155,2.018,3.943v0.197l0.202-0.197c-0.404,0.394-0.807,0.986-1.211,1.577
						c-6.053,8.083-10.088,16.759-13.115,26.222c-0.403,1.38-0.605,2.563-0.807,3.943c-0.202,1.774,0.404,3.746,1.009,3.746
						C664.783,465.033,665.793,464.837,666.599,464.442z M665.187,425.602c-1.21,0-2.018,0.592-1.816,1.774
						C664.38,427.376,664.986,426.588,665.187,425.602z M667.205,423.63c-1.009,0-1.816,0.592-1.816,1.774
						C666.398,425.208,667.003,424.616,667.205,423.63z M668.214,423.039c0-0.197,0-0.394,0.202-0.591
						c-0.605-0.197-0.807,0-1.009,0.789C667.609,423.236,667.81,423.236,668.214,423.039z M671.038,427.771h-0.202l-0.403,0.591
						L671.038,427.771z M675.679,425.405h-0.202c0.403-0.985,1.009-1.577,2.018-1.971c0.202,0,0.202,0,0.404,0.197
						c-0.404,0.986-1.009,1.774-2.22,1.972V425.405z M679.715,427.376v0.197c-0.605,0.591-1.413,1.38-1.816,1.972h-0.202
						C678.1,428.559,678.706,427.771,679.715,427.376z M700.093,525.167l-0.202,0.197l-0.404,0.591
						C699.689,525.759,699.891,525.561,700.093,525.167z M700.9,524.575c0.807-0.197,1.413-0.591,1.614-1.577
						C701.707,523.196,701.102,523.787,700.9,524.575L700.9,524.575z M706.55,522.801c0.403-0.985,1.21-1.774,2.219-2.168v-3.549
						c-2.623,1.183-4.842,2.957-6.053,5.717H706.55z M704.935,527.139h1.614c-0.202,0.789-0.807,1.38-1.614,1.774V527.139z
						 M706.953,524.97c0.403,0,1.009,0.197,1.816,0.197c-0.403,0.789-1.009,1.38-1.816,1.577V524.97z M709.576,516.492
						c0-0.197,0.202-0.394,0.202-0.591l-0.403,0.591H709.576z M781.001,557.107c-5.649-5.717-10.29-11.83-12.509-19.716c-0.404-1.972-1.009-3.943-1.211-5.915
						c-0.605-7.098-0.605-14.195,0-21.293c0.605-4.535,1.816-9.266,3.632-13.604c1.009-2.76,2.018-5.718,3.229-8.478
						c0.403-1.38-0.605-2.76-2.22-2.76h-3.026c-6.457,0.395-12.308-1.38-18.159-3.746c-7.465-2.958-13.922-7.295-19.975-12.421
						c-4.237-3.549-7.264-7.689-11.097-12.421c-2.22-2.958-3.632-6.112-4.237-9.661c0-0.591,0-1.183-0.403-1.971
						c-2.421-13.999,2.825-24.645,12.509-32.926c4.035-3.549,9.483-4.337,15.334-1.577c1.009,0.591,2.018,0.789,3.026,1.38
						c4.237,1.972,7.062,4.929,9.08,9.069c2.623,5.126,3.632,10.252,3.026,15.773c-0.605,4.929-1.816,9.661-3.632,14.195
						c-1.614,4.535-3.228,8.872-5.851,13.407c-0.202,0.591,0.807,2.366,1.412,2.563c1.009,0.197,2.018,0.592,3.027,0.592
						c3.026,0.197,6.254,0.591,9.281,0.394c7.264-0.197,14.527,0.986,21.791,1.972c0.605,0.197,1.413,0.197,1.816,0.591
						c3.229,2.563,7.062,4.338,9.08,8.083c0.605,1.183,1.009,2.563,1.009,3.943c0.202,5.323,0,10.646-1.21,15.97
						c-1.009,5.323-2.018,10.647-2.825,15.97c-0.202,1.774-0.202,3.352-0.404,5.126c-0.202,2.366-0.403,4.732-0.807,7.098
						c-1.211,5.323-1.211,10.646,0,15.97c0.605,2.76,0.807,5.323,1.21,8.083c0.403,1.38,1.009,2.563,2.219,3.352l1.816,0.591
						c0.404,0,0.807-0.197,1.009-0.197c26.432-10.252,47.617-69.4,54.679-91.679c1.614-2.563,4.438-2.958,6.86-0.789
						c0.605,0.394,0.807,1.38,0.807,1.774c-0.403,1.38-1.009,2.563-1.816,3.746c-9.483,24.448-25.625,71.174-43.783,88.918
						c-4.843,4.535-10.088,8.083-16.747,9.463C790.484,564.993,785.44,561.445,781.001,557.107z M722.287,424.616
						c0-0.394-0.202-0.986,0-1.38c-0.403,0.197-0.807,0.394-1.21,0.789c-0.202,0.197,0,0.591,0,0.789
						c0,0.395,0.202,0.789,0.404,0.789C721.884,425.799,722.287,425.405,722.287,424.616z M723.699,421.264
						c-0.807,0.197-1.21,0.591-1.413,1.577C723.296,422.645,723.699,422.053,723.699,421.264z M724.708,420.476v-0.591
						c-0.202,0.197-0.202,0.394-0.202,0.789C724.507,420.476,724.708,420.476,724.708,420.476z M729.753,421.264h-0.202
						c0,0-0.202,0.395,0,0.591C729.551,421.659,729.753,421.462,729.753,421.264z M739.639,458.725
						c0.807-1.183,1.614-2.169,2.018-3.352c2.421-6.9,5.448-13.604,6.457-21.293c0.807-4.929,0.202-9.858-1.614-14.984
						c-0.404-1.38-3.229-2.76-4.439-1.971c-1.009,0.394-1.816,0.789-2.623,1.38c-1.614,1.38-3.026,2.76-4.641,4.337l-2.219,5.323
						c-2.22,5.126-2.623,10.647-1.413,16.562l0.403,2.958c0.605,3.549,2.421,6.703,4.641,9.661l1.614,1.38
						C738.428,459.316,739.236,459.316,739.639,458.725z M731.972,420.673c-0.404-0.395-0.605-0.789-0.807-0.986
						c-0.202,0.197-0.403,0.591-0.403,0.986H731.972z M732.577,421.264c0.605,0.591,1.21,1.183,1.816,1.38
						C733.99,422.053,733.384,421.462,732.577,421.264z M334.276,734.815c-3.249-4.332-5.878-8.972-8.199-13.922c-7.425-16.397-12.995-33.259-15.779-50.895
						c-1.702-10.983-2.166-21.966-2.166-33.104c0-21.193,2.166-42.077,8.199-62.342c3.094-10.364,7.426-20.265,12.84-29.701
						c3.867-6.497,8.508-12.53,14.232-17.48c3.867-3.403,8.199-6.342,13.304-7.58c9.281-2.166,16.552,0.31,20.574,8.354
						c12.685,20.265-17.017,66.982-44.861,103.954c0.31,2.166,0.31,2.166,0.619,5.105c0.773,7.58,1.238,15.16,1.856,22.585
						c0,1.083,0.464,2.166,0.928,3.094c0.155,0.464,0.773,0.773,1.237,0.773c1.547-0.31,3.094-0.464,4.641-1.083
						c5.724-0.619,32.95-10.209,46.872-18.718c1.392-0.928,2.475-2.166,4.332-2.011c0.155,0,0.619,0,0.619,0.154
						c0.31,0.464,0.619,1.083,0.464,1.238c-0.773,1.083-1.702,2.166-2.629,3.094c-17.326,11.602-39.756,17.635-49.038,19.801
						c-0.928,0.31-1.238,2.166-0.464,2.63c1.547,1.237,3.094,2.32,4.641,3.403c12.066,9.436,19.182,21.657,20.11,37.126
						c0.464,7.116,0.154,14.077-2.321,20.883c-0.928,2.166-2.011,4.332-3.094,6.498c-5.105,8.663-16.707,8.199-21.966,3.403
						C337.369,738.527,335.823,736.671,334.276,734.815z M367.38,525.669c-11.757-5.105-23.668,19.337-28.928,39.602
						c-2.475,9.9-3.558,20.11-4.641,30.32c-1.083,10.674-2.166,29.546-1.237,29.701C357.015,592.806,389.346,535.105,367.38,525.669
						z M353.148,734.66c5.26-8.354,7.58-17.48,6.497-27.071c-1.237-11.292-6.188-21.193-13.768-29.546
						c-1.702-2.011-4.022-3.558-6.033-5.26c-0.464-0.31-1.083-0.619-1.547-0.619c-0.464,0-1.238,0.309-1.238,0.773
						c-0.464,0.928-0.464,2.011-0.619,2.939c0.464,3.403,0.773,6.497,1.392,9.746l6.033,30.939
						c0.928,4.486,1.856,8.972,3.094,13.458c0.464,2.011,1.547,3.867,2.321,5.878c0.309,0.619,1.701,0.773,2.166,0.464
						C352.065,735.743,352.684,735.279,353.148,734.66z M431.574,753.223c-2.011-1.856-3.713-4.176-4.95-6.342c-2.939-5.105-4.332-10.519-4.641-16.243
						c-0.309-5.105-0.309-10.21-0.464-12.84c0.773-24.132,2.785-45.634,8.354-66.673c0.464-1.237,0.619-2.63,0.773-3.867
						c0.155-0.31-0.464-0.773-0.773-1.083c0-0.155-0.464,0-0.619,0.31c-0.928,1.237-2.011,2.475-2.785,3.867
						c-5.105,8.663-10.055,17.326-15.005,25.989l-4.486,7.734c-1.083,2.011-2.939,3.094-5.104,3.403
						c-3.713,0.773-7.58,0.773-11.293-0.155c-7.425-1.856-11.602-6.497-12.375-14.231c-0.309-5.105-0.464-10.21,0.155-15.315
						c1.083-10.519,4.177-20.574,9.746-29.701c0.928-1.702,2.011-3.249,3.094-4.795c3.712-4.641,8.663-6.033,14.541-4.332
						c2.166,0.773,3.713,2.475,3.867,4.795c0.155,2.939-0.154,5.878-0.464,8.818c-1.392,11.292-3.094,22.431-2.63,33.878
						c0.155,1.237,0.464,2.629,0.773,3.867c0,0.31,0.928,0.619,1.392,0.464c0.619-0.464,1.393-0.928,1.856-1.547
						c1.856-2.629,3.558-5.414,5.415-8.044c4.331-6.497,8.663-12.994,13.149-19.336c3.094-4.486,6.497-8.663,9.127-13.458
						c2.32-4.332,5.414-5.415,10.209-4.486c1.856,0.464,3.403,1.702,3.713,3.868c0.464,2.32,0.619,4.795,0.464,7.116
						c-0.309,7.271-0.928,14.541-1.392,21.812c-0.155,2.166-0.464,4.331-0.619,6.497c-0.464,16.243-2.475,32.331-4.486,48.264
						c-1.083,9.9-2.166,19.956-3.094,29.856c-0.155,2.166-0.155,4.332,0.155,6.497c0.154,0.928,0.773,2.166,1.547,2.63
						c0.928,0.619,2.011-0.155,2.784-0.928c1.702-2.011,3.558-4.022,5.105-6.188c2.166-3.403,4.332-6.806,6.188-10.519
						c5.414-11.138,10.983-22.276,16.088-33.568c7.735-16.552,15.16-33.414,22.74-50.121c0.928-2.166,1.702-4.486,3.558-6.188
						c0.464-0.309,1.238-0.464,1.547-0.309c0.619,0.464,1.238,1.237,1.238,1.856c-0.31,1.547-0.773,3.094-1.392,4.486
						c-7.425,20.574-16.243,40.53-25.215,60.485c-3.867,8.353-7.734,16.707-11.757,24.905c-2.011,4.177-4.331,8.044-6.961,11.912
						c-1.702,2.63-4.022,5.105-6.497,7.116C445.032,759.566,436.524,757.709,431.574,753.223z M486.795,653.446c-0.31-13.768,4.177-25.988,13.149-36.353c5.26-6.033,11.912-10.055,19.801-11.757
						c4.332-0.928,8.663-0.773,12.685,0.928c5.724,2.166,8.354,6.806,7.425,12.839c-0.464,2.32-1.238,4.641-2.166,6.807
						c-4.95,11.602-12.375,21.038-23.823,26.453c-5.414,2.63-4.795,3.712-2.939,8.044c14.851,31.403,39.911,10.055,57.701-17.016
						c0.309-0.31,1.083-0.464,1.547-0.464c0.464,0.155,1.238,0.619,1.083,0.928c0,1.083-0.155,2.166-0.619,3.094
						c-20.574,34.651-46.408,50.43-72.706,29.856C490.972,670.617,486.95,663.037,486.795,653.446z M511.546,648.805
						c9.127-6.033,15.16-14.387,18.873-24.441c1.083-2.785,1.238-5.878,0.773-8.818c-0.619-4.95-5.259-7.116-9.436-4.486
						c-2.785,1.856-4.95,4.177-6.807,6.807c-4.95,7.271-7.425,15.469-7.58,25.679c0.155,0.773,0.31,2.785,0.928,4.95
						c0,0.31,1.083,0.773,1.856,0.773C510.618,649.424,511.237,649.115,511.546,648.805z M566.459,731.102c-4.022-8.663-6.497-17.944-8.044-27.535c-2.63-15.315-3.403-30.939-1.856-46.563
						c0.619-7.58,0.928-15.16,1.856-22.74c2.784-19.027,6.652-37.9,13.613-56.309c4.022-10.829,8.818-21.193,14.696-31.248
						c3.558-6.343,7.58-12.221,12.685-17.48c2.63-2.785,5.569-5.414,8.508-7.735c2.475-1.702,5.259-3.094,8.044-4.177
						c6.961-2.32,12.994,0.464,15.934,6.807c9.746,23.359-23.978,83.38-45.48,114.783c-0.928,3.094-2.939,33.259-2.63,44.861
						c0.31,8.044,0,16.088,0.773,24.132c4.486,40.375,9.591,40.994,24.906,1.702c3.867-10.829,7.735-21.657,11.292-32.64
						c2.785-8.508,5.26-17.326,7.735-25.834c0.773-2.32,1.856-4.641,2.939-6.807c0-0.309,0.928-0.619,1.238-0.464
						c0.464,0,0.928,0.619,0.773,1.083c-0.155,2.63-0.31,5.414-1.238,8.044c-6.806,22.74-13.922,45.48-20.883,68.22
						c-1.393,4.332-3.249,8.663-5.105,12.995c-8.199,17.326-24.441,22.43-34.033,7.271
						C570.017,738.063,568.006,734.66,566.459,731.102z M623.696,521.338c-1.393-1.393-3.403-1.547-4.95-0.464
						c-2.939,2.011-4.796,4.641-6.652,7.425c-3.868,6.342-6.807,13.149-9.437,20.11c-3.867,10.829-6.807,21.966-8.972,33.104
						c-4.641,23.668-6.033,30.938-7.425,50.121C602.657,607.656,639.784,536.033,623.696,521.338z";

    let path_parser = svgtypes::PathParser::from(svg_string);
    let path_segments = path_parser.filter_map(Result::ok).into_iter();

    points_from_path_segments(path_segments)
}

fn main() {
    let example_app = ExampleApp::new();

    // Set up the coordinate system to be fixed at 900x600, and use this as the default window size
    // This means the drawing code can be written as though the window is always 900x600. The
    // output will be automatically scaled so that it's always visible.
    let logical_size = LogicalSize::new(1000.0, 1000.0);
    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: logical_size.width as f32,
        top: 0.0,
        bottom: logical_size.height as f32,
    };
    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Center;

    skulpin::AppBuilder::new()
        .app_name(CString::new("Debug drawing").unwrap())
        .use_vulkan_debug_layer(true)
        .logical_size(logical_size)
        .coordinate_system(CoordinateSystem::VisibleRange(visible_range, scale_to_fit))
        .run(example_app);
}

struct ExampleApp {
    points_to_draw: LinkedList<LineTo>,
}

impl ExampleApp {
    pub fn new() -> Self {
        let mut linked_list = LinkedList::new();
        for point in points_to_draw() {
            linked_list.push_back(point)
        }
        ExampleApp {
            points_to_draw: linked_list,
        }
    }
}

impl AppHandler for ExampleApp {
    fn update(
        &mut self,
        app_control: &mut AppControl,
        input_state: &InputState,
        _time_state: &TimeState,
    ) {
        if input_state.is_key_down(VirtualKeyCode::Escape) {
            app_control.enqueue_terminate_process();
        }
    }

    fn draw(
        &mut self,
        _app_control: &AppControl,
        _input_state: &InputState,
        _time_state: &TimeState,
        canvas: &mut skia_safe::Canvas,
        _coordinate_system_helper: &CoordinateSystemHelper,
    ) {
        // Generally would want to clear data every time we draw
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));

        // Make a color to draw with
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1., 0., 0., 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        // Draw SVG
        let mut prev_point: Point = Point { x: 0.0, y: 0.0 };
        for points in self.points_to_draw.iter() {
            match points {
                LineTo::Fly(point) => prev_point = point.clone(),

                LineTo::Draw(point) => {
                    canvas.draw_line(
                        skia_safe::Point::new(prev_point.x as f32, prev_point.y as f32),
                        skia_safe::Point::new(point.x as f32, point.y as f32),
                        &paint,
                    );
                    prev_point = point.clone();
                }

                LineTo::Erase(_) => {}
            }
        }
    }

    fn fatal_error(&mut self, error: &skulpin::AppError) {
        println!("{}", error);
    }
}
