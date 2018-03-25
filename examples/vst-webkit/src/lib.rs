#[macro_use] extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Plugin, Info };
use vst::editor::Editor;

extern crate tinyui;
use tinyui::*;

mod ui;

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct DigiDist {
}

impl Default for DigiDist {
    fn default() -> DigiDist {
        DigiDist {
        }
    }
}
impl Editor for DigiDist {
    fn size(&self) -> (i32, i32) { (WIDTH as i32, HEIGHT as i32) }
    fn position(&self) -> (i32, i32) { (0, 0) }
    fn is_open(&mut self) -> bool { true }
    // fn close(&mut self) { self.app = None }

    fn open(&mut self, window: *mut std::os::raw::c_void) {
        let window = Window::new_with_parent(window).unwrap();
        ui::PluginWindow::new(window);
    }
}

impl Plugin for DigiDist {
    fn get_info(&self) -> Info {
        Info {
            name: "DigiDistGUI".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 666997,
            category: Category::Effect,

            inputs: 2,
            outputs: 2,
            parameters: 0,

            // preset_chunks: true,

            ..Info::default()
        }
    }

    fn get_editor(&mut self) -> Option<&mut Editor> { Some(self) }
}

plugin_main!(DigiDist);