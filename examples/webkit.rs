extern crate tinyui;
use tinyui::Window;
use tinyui::{ Color, WebView, EventHandler, Event };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct App {}

impl EventHandler for App {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);
    }
}

fn main() {
    let app = App{};

    let mut window = Window::new(app, WIDTH, HEIGHT).unwrap();
    window.set_title("Window Controls");
    window.set_background_color(Color::red());

    let mut webview = WebView::new(window.frame());
    webview.load_html_string(include_str!("vst/src/index.html"));
    webview.attach(&mut window);

    window.run(); // not necessary on vsts.
}
