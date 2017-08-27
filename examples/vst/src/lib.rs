#![allow(dead_code)]

#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info};
use vst2::editor::Editor;

extern crate tinyui;
use tinyui::Window;
use tinyui::{ WebView, Label, Rect, Color, Button };

const WIDTH: f64 = 480.;
const HEIGHT: f64 = 320.;

struct DigiDist {
    threshold: f32,
    gain: f32,
    app: Option<Window>,
    ui: WebView,
}

struct PluginWindow {
    window: Window,
    label: Label,
    button: Button,
}

impl Default for DigiDist {
    fn default() -> DigiDist {
        let mut ui = WebView::new(Rect::new(0., 0., WIDTH, HEIGHT));
        ui.load_html_string(include_str!("knob.html"));

        DigiDist {
            threshold: 1.0, // VST parameters are always 0.0 to 1.0
            gain: 1.0,
            app: None,
            ui: ui,
        }
    }
}

impl Editor for DigiDist {
    fn size(&self) -> (i32, i32) { (WIDTH as i32, HEIGHT as i32) }
    fn position(&self) -> (i32, i32) { (0, 0) }
    fn is_open(&mut self) -> bool { self.app.is_some() }
    fn close(&mut self) { self.app = None }

    fn open(&mut self, window: *mut std::os::raw::c_void) {
        let mut window = Window::attach_to(window).unwrap();

        // window.on_load(&on_load);
        window.set_title("oh hai!");
        // window.set_background_color(Color::green());

        let mut label = Label::new("hello", Rect::new(10., 10., 300., 20.));
        label.attach(&mut window);

        let mut button = Button::new("hello", Rect::new(30., 10., 150., 20.));
        button.attach(&mut window);

        // let mut webview = WebView::new(Rect::new(10., 10., 600., 400.));
        // self.ui.load_html_string("<script>function changeBackground(color) {document.body.style.background = color;}</script><BODY onload=\"changeBackground('green');\"><h1>HAHAFUCK U</h1><button style='width: 150px'>PRESS ME</button></BODY>");

        // button.on_click(Box::new(move || {
        //     label.set_text("hi");
        // }));

        // let on_file_drop = std::cell::RefCell::new(Box::new(move |path:String| {
        //     println!("file got dropped bro: {:?}", path);
        //     button.set_text("dropper");
        //     label.set_text(&path);
        // }));

        self.ui.attach(&mut window);

        button.on_click(Some(Box::new(
            move |button| {
                label.set_text("hi");
                button.set_text("hi");
            }
        )));

        // window.on_file_drop(on_file_drop);

        // window.run();
        window.setup();
        self.app = Some(window);
    }

}

impl Plugin for DigiDist {
    fn get_info(&self) -> Info {
        Info {
            name: "DigiDistGUI".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 66883344,
            category: Category::Effect,

            inputs: 2,
            outputs: 2,
            parameters: 2,

            preset_chunks: true,

            ..Info::default()
        }
    }

    fn get_preset_data(&mut self) -> Vec<u8> { Vec::new() }
    fn get_bank_data(&mut self) -> Vec<u8> { Vec::new() }
    fn load_preset_data(&mut self, data: &[u8]) { }
    fn load_bank_data(&mut self, data: &[u8]) { }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            1 => self.gain,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            // We don't want to divide by zero, so we'll clamp the value
            0 => self.threshold = value.max(0.01),
            1 => self.gain = value,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            1 => "Gain".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold * 100.0),
            1 => format!("{}", self.gain * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Split out the input and output buffers into two vectors
        let (input_buffer, mut output_buffer) = buffer.split();

        // Assume 2 channels
        if input_buffer.len() < 2 || output_buffer.len() < 2 {
            return;
        }

        // Iterate over inputs as (&f32, &f32)
        let (l, r) = input_buffer.split_at(1);
        let stereo_in = l[0].iter().zip(r[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut l, mut r) = output_buffer.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {

            if *left_in >= 0.0 {
                *left_out = left_in.min(self.threshold) / self.threshold * self.gain;
            }
            else {
                *right_out = left_in.max(-self.threshold) / self.threshold * self.gain;
            }

            *left_out = *left_in + *left_out;
            *right_out = *right_in + *right_out;
        }
    }

    fn get_editor(&mut self) -> Option<&mut Editor> { Some(self) }
}

plugin_main!(DigiDist);
