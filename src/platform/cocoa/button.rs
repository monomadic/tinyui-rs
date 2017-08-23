#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO };
use cocoa::appkit::NSButton;
use cocoa::foundation::{ NSString };
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use Rect;
use Window;

pub struct Button {
    id: id,
}

extern "C" fn onClick(this: &Object, _cmd: Sel, _: id) {
    // unsafe { this.set_ivar::<u32>("_foo", bar) ;}
    println!("clicked");
}

impl Button {
    pub fn new(text: &str, position: Rect) -> Self {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("ButtonResponder", superclass).unwrap();

        unsafe { decl.add_method(sel!(onClick:),
            onClick as extern fn(this: &Object, _: Sel, _: id)) };

        let BUTTON_RESPONDER_CLASS = decl.register();
        let responder: id = unsafe { msg_send![BUTTON_RESPONDER_CLASS, new] };

        unsafe {
            let button = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            button.setTitle_(NSString::alloc(nil).init_str(text));


            msg_send![button, setTarget:responder];
            msg_send![button, setAction:sel!(onClick:)];

            // msg_send![button, setBezeled:NO];
            // msg_send![button, setDrawsBackground:NO];
            // msg_send![button, setEditable:NO];
            // msg_send![button, setSelectable:NO];
            // msg_send![button, setStringValue:NSString::alloc(nil).init_str(text)];

            Button { id: button }
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
