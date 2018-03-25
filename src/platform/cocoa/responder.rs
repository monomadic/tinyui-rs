#![allow(non_snake_case)]
#![allow(unused_variables)]

use cocoa::base::{id};

use objc::runtime::{BOOL, YES};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;

use std::os::raw::c_void;
use std::path::PathBuf;

// use platform::platform::window::WindowEvents;

extern "C" fn setViewController(this: &mut Object, _: Sel, controller: *mut c_void) {
    unsafe {
        this.set_ivar("ViewController", controller);
    }
}

pub fn set_event_handler_contained(responder: id, handler: Handler) {
    let boxed_handler = Box::new(handler);
    unsafe {
        let handler_ptr = Box::into_raw(boxed_handler) as *const Handler as *mut c_void;
        msg_send![responder, setEventHandler: handler_ptr];
    }
}

pub extern "C" fn setEventHandler(this: &mut Object, _: Sel, handler: *mut c_void) {
    unsafe { this.set_ivar("EventHandler", handler) };
}

pub fn send_event(target: id, event: Event) {
    let window: id = unsafe { msg_send![target, window] };
    let responder: id = unsafe { msg_send![window, delegate] };

    let handler_ptr: *mut c_void = unsafe { *(*responder).get_ivar("EventHandler") };
    let mut handler: Box<EventHandler> = unsafe { Box::from_raw(handler_ptr as *mut Handler) };
    handler.handle(event);

    std::mem::forget(handler); // forget this memory so the id isn't deleted!
}

use { Handler, EventHandler, Event };
use std;

/// Invoked when the image is released
extern fn prepare_for_drag_operation(_: &Object, _: Sel, _: id) {}

extern fn dragging_entered(this: &Object, _: Sel, sender: id) -> BOOL {
    let window: id = unsafe { msg_send![sender, draggingDestinationWindow] };
    let mut handler = objc_retrieve_event_handler(window);

    let pb: id = unsafe { msg_send![sender, draggingPasteboard] };
    let files = objc_get_files_from_pasteboard(pb);

    for file in files.into_iter() {
        handler.handle(Event::DraggingEntered(file));
    }

    std::mem::forget(handler);

    YES
}

/// Invoked after the released image has been removed from the screen
extern fn perform_drag_operation(this: &Object, _: Sel, sender: id) -> BOOL {
    let window: id = unsafe { msg_send![sender, draggingDestinationWindow] };
    let mut handler = objc_retrieve_event_handler(window);

    let pb: id = unsafe { msg_send![sender, draggingPasteboard] };

    let files = objc_get_files_from_pasteboard(pb);

    for file in files.into_iter() {
        handler.handle(Event::DroppedFile(file));
    }

    std::mem::forget(handler);

    YES
}

fn objc_get_files_from_pasteboard(pasteboard: id) -> Vec<PathBuf> {
    use cocoa::foundation::NSString;
    use cocoa::appkit::NSPasteboard;
    use cocoa::foundation::NSFastEnumeration;
    use cocoa::appkit;
    use std::ffi::CStr;

    let filenames = unsafe { NSPasteboard::propertyListForType(pasteboard, appkit::NSFilenamesPboardType) };
    let mut files = Vec::new();

    for file in unsafe { filenames.iter() } {
        let f = unsafe{ NSString::UTF8String(file) };
        let path = unsafe { CStr::from_ptr(f).to_string_lossy().into_owned() };
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        files.push(pathbuf);
    };

    files
}

fn objc_retrieve_event_handler(window: id) -> Box<EventHandler> {
    let responder: id = unsafe { msg_send![window, delegate] };
    let handler_ptr: *mut c_void = unsafe { *(*responder).get_ivar("EventHandler") };
    let handler: Box<EventHandler> = unsafe { Box::from_raw(handler_ptr as *mut Handler) };
    handler
}

/// Invoked when the dragging operation is complete
extern fn conclude_drag_operation(_: &Object, _: Sel, _: id) {}

/// Invoked when the dragging operation is cancelled
extern fn dragging_exited(this: &Object, _: Sel, sender: id) {
    let window: id = unsafe { msg_send![sender, draggingDestinationWindow] };
    let mut handler = objc_retrieve_event_handler(window);
    handler.handle(Event::DraggingExited);
    std::mem::forget(handler);
}

extern fn window_closed(this: &Object, _: Sel, sender: id) {
    // let window: id = unsafe { msg_send![sender, delegate] };
    // let mut handler = objc_retrieve_event_handler(sender);

    let window: id = unsafe { msg_send![sender, object] };
    let responder: id = unsafe { msg_send![window, delegate] };
    let handler_ptr: *mut c_void = unsafe { *(*responder).get_ivar("EventHandler") };
    let mut handler: Box<EventHandler> = unsafe { Box::from_raw(handler_ptr as *mut Handler) };

    handler.handle(Event::WindowWillClose);
    std::mem::forget(handler);
}

// @property(readonly) BOOL acceptsFirstResponder;
extern "C" fn acceptsFirstResponder(_: &Object, _: Sel) -> BOOL {
    // println!("acceptsFirstResponder() hit");
    YES
}

// func acceptsFirstMouse(for event: NSEvent?) -> Bool
extern "C" fn acceptsFirstMouse(_: &Object, _: Sel, theEvent: id) -> BOOL {
    // println!("acceptsFirstMouse() hit");
    YES
}

extern "C" fn mouseEvent(this: &Object, _: Sel, mouseEvent: id) {
    use cocoa::appkit::NSEvent;
    // println!("NSEvent type: {:?}", unsafe { NSEvent::eventType(mouseEvent) });
}

extern fn did_become_active(this: &Object, _: Sel, _: id) {
    // println!("focused");
}

pub fn get_window_responder_class() -> *const Class {

    use std::sync::{Once, ONCE_INIT};

    static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("ViewResponder", superclass).unwrap();

        decl.add_ivar::<*mut c_void>("ViewController");
        decl.add_ivar::<*mut c_void>("EventHandler");

        unsafe {

            decl.add_method(sel!(setViewController:),
                            setViewController as
                            extern "C" fn(this: &mut Object, _: Sel, _: *mut c_void));

            decl.add_method(sel!(setEventHandler:),
                            setEventHandler as
                            extern "C" fn(this: &mut Object, _: Sel, _: *mut c_void));

            decl.add_method(sel!(acceptsFirstResponder),
                acceptsFirstResponder as extern fn(this: &Object, _: Sel) -> BOOL);

            decl.add_method(sel!(acceptsFirstMouse:),
                acceptsFirstMouse as extern fn(this: &Object, _: Sel, _: id) -> BOOL);

            decl.add_method(sel!(applicationDidBecomeActive:),
                            did_become_active as extern fn(&Object, Sel, id));

            // func mouseDown(with event: NSEvent)
            // https://developer.apple.com/documentation/appkit/nsresponder/1524634-mousedown
            decl.add_method(sel!(mouseDown:),
                mouseEvent as extern fn(this: &Object, _: Sel, _: id));

            decl.add_method(sel!(mouseUp:),
                mouseEvent as extern fn(this: &Object, _: Sel, _: id));

            // callbacks for drag and drop events
            decl.add_method(sel!(draggingEntered:),
                dragging_entered as extern fn(&Object, Sel, id) -> BOOL);
            decl.add_method(sel!(prepareForDragOperation:),
                prepare_for_drag_operation as extern fn(&Object, Sel, id));
            decl.add_method(sel!(performDragOperation:),
                perform_drag_operation as extern fn(&Object, Sel, id) -> BOOL);
            decl.add_method(sel!(concludeDragOperation:),
                conclude_drag_operation as extern fn(&Object, Sel, id));
            decl.add_method(sel!(draggingExited:),
                dragging_exited as extern fn(&Object, Sel, id));

            decl.add_method(sel!(windowWillClose:),
                window_closed as extern fn(&Object, Sel, id));

            RESPONDER_CLASS = decl.register();
        }
    });
    unsafe { RESPONDER_CLASS }
}