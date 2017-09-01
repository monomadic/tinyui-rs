extern crate tinyui;
use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button, WebView };
use tinyui::EventHandler;

// use std::path::PathBuf;

// struct App {
//     window: Window,
//     button: Button,
//     label: Label,
// }

// impl App {
//     pub fn new() -> Self {
//         App {
//             window: Window::new(275., 150.).unwrap(),
//             button: Button::new("hello", Rect::new(30., 10., 150., 20.)),
//             label: Label::new("hello", Rect::new(10., 10., 300., 20.)),
//         }
//     }
// }

struct App {}

impl EventHandler for App {
    fn handle(&mut self) {
        println!("handling");
    }
}

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

fn main() {
    let mut app = App{};
    // app.window.run();

    let mut window = Window::new(WIDTH, HEIGHT, &mut app).unwrap();
    // window.on_load(&on_load);
    window.set_title("oh hai!");
    window.set_background_color(Color::black());

    let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
    label.attach(&mut window);

    let mut button = Button::new("hello", Rect::new(180., 50., 60., 20.));
    button.attach(&mut window);

    let mut webview = WebView::new(window.frame());
    webview.load_html_string(include_str!("vst/src/knob.html"));
    webview.attach(&mut window);

    button.on_click(Some(Box::new(
        move |button| {
            label.set_text("hi");
            button.set_text("hi");
        }
    )));

    let on_file_drop = std::cell::RefCell::new(Box::new(move |path:String| {
        println!("file got dropped bro: {:?}", path);
        label.set_text(&path);
    }));
    window.on_file_drop(on_file_drop);
    window.setup();

    window.run(); // not necessary on vsts.
}
