extern crate tinyui;
use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button };

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

fn main() {
    // let mut app = App::new();
    // app.window.run();

    let mut window = Window::new(275., 150.).unwrap();
    // window.on_load(&on_load);
    window.set_title("oh hai!");
    window.set_background_color(Color::green());

    let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
    label.attach(&mut window);

    let mut button = Button::new("hello", Rect::new(30., 10., 150., 20.));
    button.attach(&mut window);

    window.setup();

    button.on_click(Some(Box::new(
        |button| {
            // label.set_text("hi");
            button.set_text("hi");
        }
    )));

    let on_file_drop = std::cell::RefCell::new(Box::new(move |path:String| {
        println!("file got dropped bro: {:?}", path);
        label.set_text(&path);
    }));
    window.on_file_drop(on_file_drop);

    window.run(); // not necessary on vsts.
}
