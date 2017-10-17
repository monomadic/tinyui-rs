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

use std::cell::RefCell;
use std::os::raw::c_void;

#[derive(Copy, Clone)]
pub struct Slider {
    id: id,
}

#[derive(Copy, Clone)]
pub enum SliderType {
    Linear,
    Circular,
}

use std;
extern "C" fn onSliderMove(this: &Object, _cmd: Sel, target: id) {
    let value = unsafe { msg_send![target, value] };
    send_event(target, Event::SliderUpdated(value));
}

impl Slider {
    pub fn new(value:f32, min:f32, max:f32, position:Rect) -> Self {
        
        // singleton class definition
        use std::sync::{Once, ONCE_INIT};
        static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| unsafe {
            let superclass = Class::get("NSObject").expect("slider - NSObject to exist");
            let mut decl = ClassDecl::new("SliderResponder", superclass).expect("slider - responder to declare");

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


            msg_send![slider, setSliderType:1];

            msg_send![slider, setTarget:responder];
            msg_send![slider, setAction:sel!(onMouseMove:)];

            Slider { id: slider }
        };

        slider
    }

    pub fn set_value(&mut self, value: f32) {
        unsafe { msg_send![self.id, setValue:value] };
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
