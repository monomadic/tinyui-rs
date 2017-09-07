extern crate tinyui;
use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button, WebView };
use tinyui::EventHandler;

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct App {
    title: String,
    label: Label,
    button: Button,
}

impl EventHandler for App {
    fn handle(&mut self) {
        println!("title: {:?}", self.title);
        self.label.set_text(&self.title);
        self.button.set_text("clickered");
    }
}

fn main() {
    let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
    let mut button = Button::new("hello", Rect::new(180., 50., 60., 20.));

    let app = App{
        title: "window title".to_string(),
        label: label,
        button: button,
    };

    let mut window = Window::new(app, WIDTH, HEIGHT).unwrap();
    window.set_title("oh hai!");
    window.set_background_color(Color::red());

    label.attach(&mut window);
    button.attach(&mut window);

    // let mut webview = WebView::new(window.frame());
    // webview.load_html_string(include_str!("vst/src/index.html"));
    // webview.attach(&mut window);

    window.run(); // not necessary on vsts.
}
