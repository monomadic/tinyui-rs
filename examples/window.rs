extern crate tinyui;
use tinyui::Window;
use tinyui::{ Point, Size, Label, Rect, Color, Slider, SliderType, Button, EventHandler, Event };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[allow(dead_code)]
struct App {
    label: Label,
    button: Button,
    slider: Slider,
}

impl EventHandler for App {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);
    }
}

fn main() {
    let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
    let mut button = Button::new("hello", Rect::new(180., 50., 60., 20.));
    let mut slider = Slider::new(0.2, 0., 100., Rect{
        origin: Point{ x:10., y:HEIGHT-40. },
        size: Size{ width:40., height:150.},
    });
    slider.set_slider_type(SliderType::Circular);

    let app = App{
        label: label,
        button: button,
        slider: slider,
    };

    let mut window = Window::new(app, WIDTH, HEIGHT).unwrap();
    window.set_title("Window Controls");
    window.set_background_color(Color::red());

    label.attach(&mut window);
    button.attach(&mut window);
    slider.attach(&mut window);

    window.run(); // not necessary on vsts.
}
