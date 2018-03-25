use tinyui::*;

pub struct PluginWindow {
    pub window: Window,
}

impl PluginWindow {
    pub fn new(window: Window) -> Self {
        Self {
            window: window,
        }
    }
}
