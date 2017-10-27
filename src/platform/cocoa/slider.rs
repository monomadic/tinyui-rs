#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO, class };
use cocoa::foundation::{ NSString, NSAutoreleasePool };
use objc::runtime::{ Class, Object, Sel };
use objc::declare::ClassDecl;
use Rect;
use Window;
use EventHandler;
use Handler;
use Event;
use platform::platform::responder::send_event;
use platform::platform::utils::*;

use std::cell::RefCell;
use std::os::raw::c_void;

#[derive(Copy, Clone)]
pub struct Slider {
    id: id,
}

#[derive(Copy, Clone)]
pub enum SliderStyle {
    Linear = 0,
    Circular = 1,
}

pub struct SliderBuilder {
    pub id: &'static str,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub style: SliderStyle,
    pub position: Rect,
}

impl SliderBuilder {
    pub fn build(&self) -> Slider {
        Slider::new(self.id, self.value, self.min_value, self.max_value, self.style, self.position)
    }
}

use std;
extern "C" fn onSliderMove(this: &Object, _cmd: Sel, target: id) {
    let name = unsafe { 
        let ptr:u64 = *this.get_ivar("_name");
        nsstring_decode(ptr as id)
    };

    let value = unsafe { msg_send![target, value] };
    send_event(target, Event::SliderUpdated(name, value));
}

impl Slider {
    pub fn new(name: &str, value:f32, min:f32, max:f32, style: SliderStyle, position:Rect) -> Self {
        
        // singleton class definition
        use std::sync::{Once, ONCE_INIT};
        static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| unsafe {
            let superclass = Class::get("NSObject").expect("slider - NSObject to exist");
            let mut decl = ClassDecl::new("SliderResponder", superclass).expect("slider - responder to declare");

            decl.add_ivar::<u64>("_name");

            decl.add_method(sel!(onMouseMove:),
                onSliderMove as extern fn(this: &Object, _: Sel, _: id));

            RESPONDER_CLASS = decl.register();
        });

        let responder: id = unsafe { msg_send![RESPONDER_CLASS, new] };
        let slider = unsafe {
            let slider:id = msg_send![class("NSSlider"), alloc];
            msg_send![slider, initWithFrame:position.to_nsrect()];

            msg_send![slider, setMinimumValue:min];
            msg_send![slider, setMaximumValue:max];
            msg_send![slider, setValue:value];

            msg_send![slider, setSliderType:style as u32];

            let objc_text = NSString::alloc(nil).init_str(name);
            (*responder).set_ivar("_name", objc_text as u64);

            msg_send![slider, setTarget:responder];
            msg_send![slider, setAction:sel!(onMouseMove:)];

            Slider { id: slider }
        };

        slider
    }

    pub fn set_slider_type(&mut self, value: SliderStyle) {
        unsafe { msg_send![self.id, setSliderType:value as u32] };
    }

    pub fn set_value(&mut self, value: f32) {
        unsafe { msg_send![self.id, setValue:value] };
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
