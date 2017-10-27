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
use platform::platform::utils::*;

use std::cell::RefCell;
use std::os::raw::c_void;

#[derive(Copy, Clone)]
pub struct Button {
    id: id,
}

#[derive(Copy, Clone)]
pub enum ButtonStyle {
    None = 0,
    Rounded = 1,
    Square = 2,
    Disclosure = 5,
    ShadowlessSquare = 6,
    Circular = 7,
    TexturedSquare = 8,
    Help = 9,
    SmallSquare = 10,
    TexturedRounded = 11,
    RoundedRect = 12,
    Recessed = 13,
    RoundedDisclosure = 14,
    Inline = 15,
}

use std;
extern "C" fn onButtonClick(this: &Object, _cmd: Sel, target: id) {

    let name = unsafe { 
        let ptr:u64 = *this.get_ivar("_name");
        nsstring_decode(ptr as id)
    };

    send_event(target, Event::ButtonClicked(name));
}

impl Button {
    pub fn new(name: &str, text: &str, style: ButtonStyle, position: Rect) -> Self {
        
        // singleton class definition
        use std::sync::{Once, ONCE_INIT};
        static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| unsafe {
            let superclass = Class::get("NSObject").unwrap();
            let mut decl = ClassDecl::new("ButtonResponder", superclass).unwrap();

            // decl.add_ivar::<String>("ButtonState");
            decl.add_ivar::<u64>("_name");

            // extern fn objc_set_name(this: &mut Object, _cmd: Sel, ptr: u64) {
            //     unsafe {this.set_ivar("_name", ptr);}
            // }

            decl.add_method(sel!(onButtonClick:),
                onButtonClick as extern fn(this: &Object, _: Sel, _: id));

            // decl.add_method(sel!(setName:),
            //     objc_set_name as extern fn(&mut Object, Sel, u64));

            RESPONDER_CLASS = decl.register();
        });

        let responder: id = unsafe { msg_send![RESPONDER_CLASS, new] };
        let button = unsafe {
            let button = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            button.setTitle_(NSString::alloc(nil).init_str(text));

            msg_send![button, setBezelStyle:style as u32];

            msg_send![button, setTarget:responder];
            msg_send![button, setAction:sel!(onButtonClick:)];

            let objc_text = NSString::alloc(nil).init_str(name);
            (*responder).set_ivar("_name", objc_text as u64);

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
