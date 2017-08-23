extern crate tinyui;

use tinyui::Window;

// struct Poo {
//     title: String,
// }

fn main() {
    let on_load = || {
        println!("loaded window.");
    };

    let mut window = Window::new().unwrap();
    window.on_load(&on_load);
    window.set_title("oh hai!");
    window.run();
}
