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

use std::cell::RefCell;
use std::os::raw::c_void;

pub struct Button {
    id: id,
    responder: id,
    on_click_callback: Option<Box<FnMut(&mut Button) + 'static>>,
}

extern "C" fn onButtonClick(this: &Object, _cmd: Sel, target: id) {
    println!("onButtonClick called");
    // unsafe { msg_send![target, setTitle:NSString::alloc(nil).init_str("clicked")]};

    // let event_ptr: *mut c_void = unsafe { *this.get_ivar("ViewController") };
    // let events: &mut Button = unsafe { &mut *(event_ptr as *mut Button) };

    // events.click();

    // let event_handler_ptr: *mut c_void = unsafe { *this.get_ivar("EventHandler") };
    // let event_handler: &mut Handler = unsafe { &mut *(event_handler_ptr as *mut Handler) };
    // event_handler.handle();

    let event_handler_ptr: *mut c_void = unsafe { *this.get_ivar("EventHandler") };
    let event_handler = unsafe { &mut *(event_handler_ptr as *mut Handler) };
    event_handler.handle();

}

extern "C" fn setViewController(this: &mut Object, _: Sel, controller: *mut c_void) {
    println!("setViewController called");
    unsafe { this.set_ivar("ViewController", controller) };
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

            decl.add_ivar::<*mut c_void>("ViewController");
            decl.add_ivar::<*mut c_void>("EventHandler");

            decl.add_method(sel!(setViewController:),
                setViewController as
                extern "C" fn(this: &mut Object, _: Sel, _: *mut c_void));

            use platform::platform::responder::setEventHandler;
            decl.add_method(sel!(setEventHandler:),
                            setEventHandler as
                            extern "C" fn(this: &mut Object, _: Sel, _: *mut c_void));

            decl.add_method(sel!(onButtonClick:),
                onButtonClick as extern fn(this: &Object, _: Sel, _: id));

            RESPONDER_CLASS = decl.register();
        });

        // let events = Box::new(ButtonEvents{
        //     on_click_callback: None,
        //     title: "hihihi".to_string(),
        // });

        let responder: id = unsafe { msg_send![RESPONDER_CLASS, new] };
        let button = unsafe {

            let button = NSButton::alloc(nil).initWithFrame_(position.to_nsrect());
            button.setTitle_(NSString::alloc(nil).init_str(text));

            msg_send![button, setTarget:responder];
            msg_send![button, setAction:sel!(onButtonClick:)];

            Button { id: button, responder: responder, on_click_callback: None }
        };

        button
    }

    pub fn set_handler<EH:EventHandler>(&mut self, mut handler: &mut EH) {
        let handler_ptr: *mut c_void = &mut handler as *mut _ as *mut c_void;
        // unsafe { msg_send![self.responder, setEventHandler: handler_ptr] };

        unsafe { msg_send![self.responder, setEventHandler: handler_ptr as *mut c_void] };
    }

    pub fn on_click(&mut self, callback: Option<Box<FnMut(&mut Button) + 'static>>) {
        self.on_click_callback = callback;

        let button_ptr: *mut c_void = self as *mut _ as *mut c_void;
        unsafe { msg_send![self.responder, setViewController: button_ptr] };
    }

    // fn click(&mut self) {
    //     // println!("{:?}", self.title);
    //     // println!("hi");
    //     let mut callback = self.on_click_callback.take();
    //     if let Some(ref mut func) = callback {
    //         func(self);
    //         // let ref mut click = *(callback.borrow_mut());
    //         // click();
    //     }
    // }

    pub fn set_text(&mut self, text: &str) {
        unsafe { self.id.setTitle_(NSString::alloc(nil).init_str(text)) };
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
