#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO };
use cocoa::appkit::{ NSButton };
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
pub struct Button {
    id: id,
}

use std;
extern "C" fn onButtonClick(this: &Object, _cmd: Sel, target: id) {
    send_event(target, Event::ButtonClicked);
}

impl Button {
    pub fn new(text: &str, position: Rect) -> Self {
        
        // singleton class definition
        use std::sync::{Once, ONCE_INIT};
        static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| unsafe {
            let superclass = Class::get("NSObject").unwrap();
            let mut decl = ClassDecl::new("ButtonResponder", superclass).unwrap();

            decl.add_method(sel!(onButtonClick:),
                onButtonClick as extern fn(this: &Object, _: Sel, _: id));

            RESPONDER_CLASS = decl.register();
        });

        let responder: id = unsafe { msg_send![RESPONDER_CLASS, new] };
        let button = unsafe {
            let button = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            button.setTitle_(NSString::alloc(nil).init_str(text));

            msg_send![button, setTarget:responder];
            msg_send![button, setAction:sel!(onButtonClick:)];

            Button { id: button }
        };

        button
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe { self.id.setTitle_(NSString::alloc(nil).init_str(text)) };
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
