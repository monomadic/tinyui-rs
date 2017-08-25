#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO };
use cocoa::appkit::{ NSButton };
use cocoa::foundation::{ NSString, NSAutoreleasePool };
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use Rect;
use Window;

pub struct Button {
    id: id,
}

impl Drop for Button {
    fn drop(&mut self) {
        unsafe { msg_send![self.id, removeFromSuperview] };
        unsafe { msg_send![self.id, release] };
    }
}

extern "C" fn onClick(this: &Object, _cmd: Sel, _: id) {
    println!("clicked");
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

            decl.add_method(sel!(onClick:),
                onClick as extern fn(this: &Object, _: Sel, _: id));

            RESPONDER_CLASS = decl.register();
        });

        unsafe {
            let responder: id = msg_send![RESPONDER_CLASS, new];

            let button = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            button.setTitle_(NSString::alloc(nil).init_str(text));

            msg_send![button, setTarget:responder];
            msg_send![button, setAction:sel!(onClick:)];

            Button { id: button }
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
