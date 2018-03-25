extern crate tinyui;
use tinyui::Window;
use tinyui::{ App, Size, EventHandler, Event, WindowBuilder, WindowStyle };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct MyWindow {
    window: Window,
}

impl EventHandler for MyWindow {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);
        match event {
            Event::WindowWillClose => App::quit(),
            _ => (),
        }
    }
}

fn main() {
    let app = MyWindow {
        window: WindowBuilder {
            title: "Window Controls Example",
            style: WindowStyle::Default,
            size: Size { width: WIDTH, height: HEIGHT },
        }.build(),
    };

    app.window.set_handler(app);

    App::run(); // not necessary on vsts.
}
