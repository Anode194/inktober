use nannou::color::*;
use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
    println!("Hello, world!");
}
struct Model {}
fn model(_app: &App) -> Model {
    Model {}
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}

struct Fish {
    center: Point2,
    tail_b: Point2,
    tail_t: Point2,

    head_x: f32,
    head_y: f32,
    head_w: f32,
    head_h: f32,
    color: Rgb<u8>,
}
impl Fish {
    fn new() -> Fish {
        Fish {
            head_x: 80.0,
            head_y: 0.0,
            head_w: 180.0,
            head_h: 90.0,
            center: pt2(0.0, 0.0),
            tail_b: pt2(-120.0, -60.0),
            tail_t: pt2(-120.0, 60.0),
            color: rgb_u32(0x8d0054),
        }
    }
}
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    //background from hex
    let bg = rgb_u32(0x001726);
    let f = Fish::new();
    draw.background().color(bg);
    draw.tri()
        .color(f.color)
        .points(f.tail_b, f.tail_t, f.center);
    draw.ellipse()
        .color(f.color)
        .x_y(f.head_x, f.head_y)
        .w(f.head_w)
        .h(f.head_h);
    //eyeball
    draw.ellipse().color(CYAN).x_y(120.0, 8.0).w(10.0).h(10.0);
    //gills
    draw.tri()
        .points(pt2(100.0, 0.0), pt2(70.0, 8.0), pt2(70.0, -8.0))
        .color(BLUE);
    draw.text("a fish named todd")
        .x_y(0.0, 200.0)
        .font_size(18);

    draw.to_frame(app, &frame).unwrap();
}
