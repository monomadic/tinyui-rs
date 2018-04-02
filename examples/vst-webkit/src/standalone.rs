extern crate tinyui;
use tinyui::*;

mod ui;

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

fn main() {
    let window = WindowBuilder {
        title: "Vst Plugin Example Standalone",
        style: WindowStyle::Default,
        size: Size { width: WIDTH, height: HEIGHT },
    }.build().expect("window to build correctly");

    let _ = ui::PluginWindow::new(window);
    let _ = App::run(); // start a cocoa runloop. not necessary on vsts.
}
