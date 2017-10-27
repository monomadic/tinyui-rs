extern crate tinyui;
use tinyui::Window;
use tinyui::{ App, Point, Size, Label, Rect, SliderBuilder, Slider,
    SliderStyle, ButtonBuilder, Button, ButtonStyle, EventHandler, Event, WindowBuilder, WindowStyle };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[allow(dead_code)]
struct MyWindow {
    window: Window,
    label: Label,
    button: Button,
    slider: Slider,
    slider_label: Label,
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
        slider: SliderBuilder {
            id: "a slider",
            value: 0.5,
            min_value: 0.0,
            max_value: 1.0,
            style: SliderStyle::Circular,
            position: Rect {
                origin: Point{ x:10., y:HEIGHT-80. },
                size: Size{ width:40., height:40.}
            }}.build(),
        slider_label: Label::new("0.5", Rect::new(25., HEIGHT-230., 40., 150.)),
        window: WindowBuilder {
            title: "Window Controls Example",
            style: WindowStyle::Default,
            size: Size { width: WIDTH, height: HEIGHT },
        }.build(),
    };

    app.label.attach(&mut app.window);
    app.button.attach(&mut app.window);
    app.slider.attach(&mut app.window);
    app.slider_label.attach(&mut app.window);

    app.window.set_handler(app);

    let _ = App::run(); // not necessary on vsts.
}
