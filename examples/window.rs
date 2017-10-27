extern crate tinyui;
use tinyui::Window;
use tinyui::{ Point, Size, Label, Rect, Color, Slider, SliderType, Button, ButtonStyle, EventHandler, Event };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[allow(dead_code)]
struct App {
    label: Label,
    button: Button,
    slider: Slider,
    slider_label: Label,
}

impl EventHandler for App {
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
    let mut label = Label::new("Cocoa Controls Demo", Rect::new(10., HEIGHT - 30., 300., 20.));
    let mut button = Button::new("a button", "click me", ButtonStyle::Square, Rect::new(10., 50., 150., 50.));
    let mut slider = Slider::new("my slider", 0.2, 0., 100., Rect{
        origin: Point{ x:10., y:HEIGHT-80. },
        size: Size{ width:40., height:150.},
    });
    let mut slider_label = Label::new("0.1", Rect::new(25., HEIGHT-230., 40., 150.));
    slider.set_slider_type(SliderType::Circular);

    let app = App{
        label: label,
        button: button,
        slider: slider,
        slider_label: slider_label,
    };

    let mut window = Window::new(app, WIDTH, HEIGHT).unwrap();
    window.set_title("Window Controls");
    // window.set_background_color(Color::white());

    label.attach(&mut window);
    button.attach(&mut window);
    slider.attach(&mut window);
    slider_label.attach(&mut window);

    window.run(); // not necessary on vsts.
}
