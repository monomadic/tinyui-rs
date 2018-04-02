extern crate tinyui;
use tinyui::Window;
use tinyui::{ App, Size, Label, Rect, SliderBuilder, Slider,
    SliderStyle, ButtonBuilder, Button, ButtonStyle, EventHandler, Event, WindowBuilder, WindowStyle };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[allow(dead_code)]
struct MyWindow {
    window: Window,
    label: Label,
    button: Button,
    button_on: Button,
    slider: Slider,
    slider_label: Label,
}

impl EventHandler for MyWindow {
    fn handle(&mut self, event: Event) {
        println!("-- event: {:?}", event);

        match event {
            Event::ButtonClicked(name) => {
                match name.as_str() {
                    "a button" => { self.button.set_text("clicked me"); self.window.set_background_color(tinyui::Color::red()) },
                    "b button" => { self.button_on.set_text("clicked me too") ; self.window.set_background_color(tinyui::Color::green()) },
                    _ => ()
                }
            },
            Event::SliderUpdated(_, val) => self.slider_label.set_text(&format!("{:.2}", val)),
            Event::WindowWillClose => App::quit(),
            _ => ()
        }
    }
}

fn main() {
    let window_rect = Rect::new(0., 0., HEIGHT, WIDTH);
    let (top_half_rect, bottom_half_rect) = window_rect.split_horizontal();

    let mut app = MyWindow {
        label: Label::new("Cocoa Controls Demo", top_half_rect),
        slider: SliderBuilder {
            id: "a slider",
            value: 0.5,
            min_value: 0.0,
            max_value: 1.0,
            style: SliderStyle::Circular,
            position: top_half_rect}.build(),
        slider_label: Label::new("0.5", top_half_rect),
        button: ButtonBuilder {
            id: "a button",
            text: "click me",
            style: ButtonStyle::Square,
            position: bottom_half_rect.inset(10.) }.build(),
        button_on: ButtonBuilder {
            id: "b button",
            text: "click me",
            style: ButtonStyle::Square,
            position: top_half_rect.inset(10.) }.build(),
        window: WindowBuilder {
            title: "Window Controls Example",
            style: WindowStyle::Default,
            size: Size { width: WIDTH, height: HEIGHT },
        }.build().expect("window did not create correctly"),
    };

    // app.label.attach(&mut app.window);
    app.button.attach(&mut app.window);
    app.button_on.attach(&mut app.window);
    // app.slider.attach(&mut app.window);
    // app.slider_label.attach(&mut app.window);

    app.window.set_handler(app);

    let _ = App::run(); // not necessary on vsts.
}
