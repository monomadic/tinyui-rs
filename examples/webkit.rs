extern crate tinyui;
use tinyui::Window;
use tinyui::{ Color, WebView, EventHandler, Event, WindowBuilder, WindowStyle, Size };

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

    let mut window = WindowBuilder {
            title: "Webkit Example",
            style: WindowStyle::Default,
            size: Size { width: WIDTH, height: HEIGHT },
        }.build();

    window.set_background_color(Color::white());

    let mut webview = WebView::new(window.frame());
    webview.load_html_string(include_str!("vst-webkit/src/index.html"));
    webview.attach(&mut window);

    window.set_handler(app);
    window.run(); // not necessary on vsts.
}
