use tinyui::*;

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

#[derive(Clone, Copy)]
pub struct PluginWindow {
    window: Window,
    button: Button,
}

impl EventHandler for PluginWindow {
    fn handle(&mut self, event: Event) {
        match event {
            Event::WindowWillClose => App::quit(),
            _ => (),
        }
    }
}

impl PluginWindow {
    pub fn new(mut window: Window) -> Self {
        let window_rect = Rect::new(0., 0., HEIGHT, WIDTH);
        let (_top_half_rect, bottom_half_rect) = window_rect.split_horizontal();

        let mut app = Self {
            window: window,
            button: ButtonBuilder {
                id: "a button",
                text: "click me",
                style: ButtonStyle::Square,
                position: bottom_half_rect.inset(10.) }.build(),
        };

        window.set_background_color(Color::red());
        app.button.attach(&mut app.window);
        app.window.set_handler(app.clone());

        app
    }
}
