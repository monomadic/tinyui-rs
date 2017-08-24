extern crate tinyui;
use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button };

// use std::path::PathBuf;

use tinyui::{ ViewController, Controller };

struct App {
    // window: Window,
    label: Label,
}

impl App {
    fn new() -> App {
       let on_load = || {
            println!("loaded window.");
        };

        let controller = MyController{};

        let mut controller_ref = Controller::new(Box::new(controller));

        let mut window = Window::new(275., 150., &mut controller_ref).unwrap();
        window.on_load(&on_load);
        window.set_title("oh hai!");
        window.set_background_color(Color::green());

        let mut label = Label::new("hello", Rect::new(10., 10., 150., 20.));
        label.attach(&mut window);

        let mut button = Button::new("hello", Rect::new(30., 10., 150., 20.));
        button.attach(&mut window);

        pub struct MyController {}
        impl ViewController for MyController {
            fn on_mouse_down(&mut self) {
                println!("Yaaaas!!");
            }

            fn on_file_drop(&mut self, path: String) {
                println!("I droppperdd a file: {:?}", path);
                label.set_text(path);
            }
        }


    }

    fn run(&mut self) {
        self.window.run();
        println!("done");

    }
}

fn main() {
    let app = App::new();
    app.run();
}
