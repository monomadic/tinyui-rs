use tinyui::*;

pub struct PluginWindow {
    pub window: Window,
}

impl PluginWindow {
    pub fn new(mut window: Window) -> Self {
        window.set_background_color(Color::red());
        Self {
            window: window,
        }
    }
}
