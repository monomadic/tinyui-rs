extern crate tinyui;

use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button };

// struct Poo {
//     title: String,
// }

fn main() {
    let on_load = || {
        println!("loaded window.");
    };

    let mut window = Window::new(275., 150.).unwrap();
    window.on_load(&on_load);
    window.set_title("oh hai!");
    window.set_background_color(Color::green());

    let mut label = Label::new("hello", Rect::new(10., 10., 150., 20.));
    label.attach(&mut window);

    let mut button = Button::new("hello", Rect::new(30., 10., 150., 20.));
    button.attach(&mut window);

    window.run();
}
