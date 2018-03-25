extern crate tinyui;
use tinyui::*;

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

fn main() {
    let window_rect = Rect::new(0., 0., HEIGHT, WIDTH);
    let window = WindowBuilder {
        title: "Window Controls Example",
        style: WindowStyle::Default,
        size: Size { width: WIDTH, height: HEIGHT },
    }.build();

    let _ = App::run(); // not necessary on vsts.
}
