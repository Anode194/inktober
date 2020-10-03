use nannou::color::*;
use nannou::prelude::*;
use nannou::geom::rect::Rect;
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
    println!("Hello, world!");
}
struct Model {}
fn model(_app: &App) -> Model {
    Model {}
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let bg = rgb_u32(0x001726);
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();   

    let x = map_range(sine, -1.0,1.0, boundary.left()+80.0, boundary.right() -80.0);
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom()+80.0, boundary.top()-80.0);
    draw.background().color(bg);
    draw.ellipse()
        .color(WHITE)
        .w_h(70.0,60.0)
        .x_y(x,y);
    //    let sheet:Rect = Rect::from_x_y_w_h(x,y,15.0,45.0);
    let sheet_y = y -25.0;
    draw.rect()
        .w_h(70.0,45.0)
        .x_y(x,sheet_y);
    draw.text("wisp oct 2 2020")
        .x_y(0.0, 200.0)
        .font_size(18);
    let left_eye = x - 10.0;
    let right_eye = x + 10.0;


    draw.rect()
        .w_h(5.0,10.0)
        .x_y(left_eye,y)
        .color(bg);
    draw.rect()
        .w_h(5.0,10.0)
        .x_y(right_eye,y)
        .color(bg);
    

    draw.to_frame(app, &frame).unwrap();

}
