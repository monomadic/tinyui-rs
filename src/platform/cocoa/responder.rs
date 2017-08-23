#![allow(non_snake_case)]
#![allow(unused_variables)]

// use cocoa;
use cocoa::base::{id};

// use objc::runtime;
use objc::runtime::{BOOL, YES};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;

// use std::os::raw::c_void;

// use window::view_controller::*;

pub fn get_window_responder_class() -> *const Class {

    use std::sync::{Once, ONCE_INIT};

    static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("ViewResponder", superclass).unwrap();

        // decl.add_ivar::<*mut c_void>("ViewController");

        // extern "C" fn setViewController(this: &mut Object, _: Sel, controller: *mut c_void) {
        //     unsafe {
        //         this.set_ivar("ViewController", controller);
        //     }
        // }


        /// Invoked when the image is released
        extern fn prepare_for_drag_operation(_: &Object, _: Sel, _: id) {}

        extern fn dragging_entered(this: &Object, _: Sel, sender: id) -> BOOL { YES }

        /// Invoked after the released image has been removed from the screen
        extern fn perform_drag_operation(this: &Object, _: Sel, sender: id) -> BOOL {
            use cocoa::appkit::NSPasteboard;
            use cocoa::foundation::NSFastEnumeration;
            use std::path::PathBuf;
            use cocoa::appkit;
            use std::os::raw::c_void;

            let pb: id = unsafe { msg_send![sender, draggingPasteboard] };
            let filenames = unsafe { NSPasteboard::propertyListForType(pb, appkit::NSFilenamesPboardType) };

            for file in unsafe { filenames.iter() } {
                use cocoa::foundation::NSString;
                use std::ffi::CStr;

                unsafe {
                    let f = NSString::UTF8String(file);
                    let path = CStr::from_ptr(f).to_string_lossy().into_owned();

                    // let state: *mut c_void = *this.get_ivar("winitState");
                    // let state = &mut *(state as *mut DelegateState);
                    // emit_event(state, WindowEvent::DroppedFile(PathBuf::from(path)));
                    println!("Dropped file: {:?}", PathBuf::from(path));
                }
            };

            YES
        }

        /// Invoked when the dragging operation is complete
        extern fn conclude_drag_operation(_: &Object, _: Sel, _: id) {}

        /// Invoked when the dragging operation is cancelled
        extern fn dragging_exited(this: &Object, _: Sel, _: id) {
            // unsafe {
            //     let state: *mut c_void = *this.get_ivar("winitState");
            //     let state = &mut *(state as *mut DelegateState);
            //     emit_event(state, WindowEvent::HoveredFileCancelled);
            // }
        }

        // @property(readonly) BOOL acceptsFirstResponder;
        extern "C" fn acceptsFirstResponder(_: &Object, _: Sel) -> BOOL {
            println!("acceptsFirstResponder() hit");
            YES
        }

        // func acceptsFirstMouse(for event: NSEvent?) -> Bool
        extern "C" fn acceptsFirstMouse(_: &Object, _: Sel, theEvent: id) -> BOOL {
            println!("acceptsFirstMouse() hit");
            YES
        }

        extern "C" fn mouseEvent(this: &Object, _: Sel, mouseEvent: id) {
            use cocoa::appkit::NSEvent;
            println!("NSEvent type: {:?}", unsafe { NSEvent::eventType(mouseEvent) });
            // Note: to get raw event type (for events unsupported by cocoa-rs),
            // let event: u64 = unsafe { msg_send![mouseEvent, type] };
            // println!("type: {}", event);

            // let view_controller: *mut c_void = unsafe { *this.get_ivar("ViewController") };
            // let view_controller = unsafe { &mut *(view_controller as *mut Controller) };
            // view_controller.on_mouse_down();
            //FIXME: view_controller needs to be a *mut ViewController trait object
            //that points to a use supplied type that implements ViewController.
            //The current setup is a hack that implements ViewController on &mut c_void
        }

        extern fn did_become_active(this: &Object, _: Sel, _: id) {
            unsafe {
                println!("focused");
            }
        }

        // decl.add_method(sel!(setViewController:),
        //                 setViewController as
        //                 extern "C" fn(this: &mut Object, _: Sel, _: *mut c_void));

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

        RESPONDER_CLASS = decl.register();
    });
    unsafe { RESPONDER_CLASS }
}