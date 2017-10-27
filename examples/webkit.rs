extern crate tinyui;
use tinyui::Window;
use tinyui::{ Color, WebView, EventHandler, Event };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct App {}

impl EventHandler for App {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);
        match event {
            Event::WebEvent(t,n) => {
                match t.as_str() {
                    "notification" => println!("yarrrr: {}", n),
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let app = App{};

    let mut window = Window::new(app, WIDTH, HEIGHT).unwrap();
    window.set_title("Window Controls");
    window.set_background_color(Color::red());

    let mut webview = WebView::new(window.frame());
    webview.load_html_string(include_str!("vst-webkit/src/index.html"));
    webview.attach(&mut window);

    window.run(); // not necessary on vsts.
}
