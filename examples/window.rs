extern crate tinyui;
use tinyui::Window;
use tinyui::{ App, Point, Size, Label, Rect, Slider, SliderType, ButtonBuilder, Button, ButtonStyle, EventHandler, Event };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[allow(dead_code)]
struct MyWindow {
    label: Label,
    button: Button,
    slider: Slider,
    slider_label: Label,
    window: Window,
}

impl EventHandler for MyWindow {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);

        match event {
            Event::ButtonClicked(name) => {
                match name.as_str() {
                    "a button" => self.button.set_text("clicked me"),
                    _ => ()
                }
            },
            Event::SliderUpdated(_, val) => self.slider_label.set_text(&format!("{:.2}", val)),
            _ => ()
        }
    }
}

fn main() {
    let mut app = MyWindow {
        label: Label::new("Cocoa Controls Demo", Rect::new(10., HEIGHT - 30., 300., 20.)),
        button: ButtonBuilder {
            id: "a button",
            text: "click me",
            style: ButtonStyle::Square,
            position: Rect::new(10., 50., 150., 50.)}.build(),
        slider: Slider::new("my slider", 0.2, 0., 100., Rect{
            origin: Point{ x:10., y:HEIGHT-80. },
            size: Size{ width:40., height:150.}}),
        slider_label: Label::new("0.1", Rect::new(25., HEIGHT-230., 40., 150.)),
        window: Window::new(WIDTH, HEIGHT).unwrap(),
    };

    app.window.set_title("Window Controls");
    app.label.attach(&mut app.window);
    app.button.attach(&mut app.window);
    app.slider.attach(&mut app.window);
    app.slider_label.attach(&mut app.window);
    app.slider.set_slider_type(SliderType::Circular);

    app.window.set_handler(app);

    let _ = App::run(); // not necessary on vsts.
}
