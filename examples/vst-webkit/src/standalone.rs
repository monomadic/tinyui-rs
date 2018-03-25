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
    }.build();

    let _plugin_window = ui::PluginWindow::new(window);
    let _ = App::run(); // not necessary on vsts.
}
