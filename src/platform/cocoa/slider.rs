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

use std;
extern "C" fn onSliderMove(this: &Object, _cmd: Sel, target: id) {
    send_event(target, Event::SliderUpdated(0.0));
}

impl Slider {
    pub fn new(value: f32, position: Rect) -> Self {
        
        // singleton class definition
        use std::sync::{Once, ONCE_INIT};
        static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = ONCE_INIT;

        // INIT.call_once(|| unsafe {
        //     let superclass = Class::get("NSObject").expect("slider - NSObject to exist");
        //     let mut decl = ClassDecl::new("ButtonResponder", superclass).expect();

        //     decl.add_method(sel!(onButtonClick:),
        //         onSliderMove as extern fn(this: &Object, _: Sel, _: id));

        //     RESPONDER_CLASS = decl.register();
        // });

        let responder: id = unsafe { msg_send![RESPONDER_CLASS, new] };
        let slider = unsafe {
            let slider:id = msg_send![class("NSSlider"), alloc];
            msg_send![slider, initWithFrame:position.to_nsrect()];

            // let slider = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            // button.setTitle_(NSString::alloc(nil).init_str(text));

            // msg_send![button, setTarget:responder];
            // msg_send![button, setAction:sel!(onButtonClick:)];

            Slider { id: slider }
        };

        slider
    }

    pub fn set_value(&mut self, value: f32) {
        // unsafe { self.id.setTitle_(NSString::alloc(nil).init_str(text)) };
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
