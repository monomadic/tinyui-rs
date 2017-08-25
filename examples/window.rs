extern crate tinyui;
use tinyui::Window;
use tinyui::{ Label, Rect, Color, Button };

// use std::path::PathBuf;

fn main() {
    let mut window = Window::new(275., 150.).unwrap();
    // window.on_load(&on_load);
    window.set_title("oh hai!");
    window.set_background_color(Color::green());

    let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
    label.attach(&mut window);

    let mut button = Button::new("hello", Rect::new(30., 10., 150., 20.));
    button.attach(&mut window);

    window.setup();
    window.on_file_drop(Box::new(move|path| {
        println!("file got dropped bro: {:?}", path);
        label.set_text(&path);
    }));

    window.run(); // not necessary on vsts.
}
