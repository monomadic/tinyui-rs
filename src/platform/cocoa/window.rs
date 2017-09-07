#![allow(dead_code)]

use cocoa::base::{ id, nil };
use std::os::raw::c_void;
use std::cell::RefCell;

use cocoa::base::{ NO, YES };
use cocoa::foundation::{ NSString, NSRect, NSSize, NSPoint, NSAutoreleasePool };
use cocoa::appkit::{ NSApp, NSApplication, NSWindow, NSView, NSTitledWindowMask, NSClosableWindowMask, NSResizableWindowMask, NSBackingStoreBuffered, NSRunningApplication,
                     NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular, NSFilenamesPboardType };

use Color;
use Rect;

pub struct Window {
    pub nswindow: id,
    pub nsview: id,
    // handler: &'a H,
    events: Box<WindowEvents>,
}

pub trait EventHandler {
    fn handle(&mut self);
}

pub struct Handler {
    pub handler: Box<EventHandler>,
}

impl EventHandler for Handler {
    fn handle(&mut self) {
        self.handler.handle();
    }
}

pub struct WindowEvents {
    pub on_file_drop_callback: Option<RefCell<Box<FnMut(String)>>>,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { self.nsview.removeFromSuperview() };
    }
}

impl WindowEvents {
    pub fn on_mouse_down(&mut self) {
        println!("Yaaaas!!");
    }

    pub fn on_file_drop(&mut self, path: String) {
        if let Some(ref mut callback) = self.on_file_drop_callback {
            let ref mut file_drop = *(callback.borrow_mut());
            file_drop(path);
        }
    }
}

// struct Holder<'a>(&'a mut (EventHandler + 'a));
// static mut static_state: *mut Holder<'static> = 0 as *mut _;
// use std;

impl Window {

    /// Create a new Window from scratch.
    pub fn new<H:'static + EventHandler>(width: f64, height: f64, handler: H) -> Result<Window, String> {
        // callback();

        // set the responder class delegate
        use platform::platform::responder::*;
        let responder: id = unsafe { msg_send![get_window_responder_class(), new] };

        // handler.handle();
        // let handler_ptr: *mut c_void = &mut handler as *mut _ as *mut c_void;
        // unsafe { msg_send![responder, setEventHandler: handler_ptr]; }

        set_event_handler_contained(responder, Handler{ handler: Box::new(handler) });
        // let h = Holder(handler);
        // unsafe {
        //     // Straight-up lie to the compiler: "yeah, this is static"
        //     static_state = std::mem::transmute(&h);
        //     // c_api(my_callback_impl);
        //     msg_send![responder, setEventHandler: static_state];
        //     static_state = 0 as *mut _;
        // }

        // test
        println!("{:?}", responder);
        // unsafe { msg_send![responder, testHandler]; }

        let window = unsafe { NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(width, height)),
                                                          NSTitledWindowMask | NSClosableWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO) };

        let view = unsafe { NSWindow::contentView(window) };

        unsafe { msg_send![window, setDelegate:responder] };
        let r: id = unsafe { msg_send![window, delegate] };
        // println!("{:?}", (r, responder));
        // unsafe { msg_send![r, testHandler]; }

        let events = Box::new(WindowEvents{
            on_file_drop_callback: None,
        });

        unsafe {
            //            let _pool = NSAutoreleasePool::new(nil);

            window.setAcceptsMouseMovedEvents_(YES); // msg_send![window, setAcceptsMouseMovedEvents: YES];
            window.makeKeyAndOrderFront_(nil);
            // window.setContentView_(view);
            window.makeFirstResponder_(view);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            app.activateIgnoringOtherApps_(YES);

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            window.setOpaque_(YES);
            window.center();

            use cocoa::foundation::NSArray;
            // register for drag and drop operations.
            msg_send![window,
                registerForDraggedTypes:NSArray::arrayWithObject(nil, NSFilenamesPboardType)];
        }



        Ok(Window {
            nswindow: window,
            nsview: view,
            events: events,
            // handler: handler,
        })
    }

    // pub fn set_handler<EH:EventHandler>(&mut self, handler: &EH) {
    //     let handler_ptr: *mut c_void = &mut handler as *mut _ as *mut c_void;
        
    //     unsafe {
    //         msg_send![self.responder, setEventHandler: event_ptr];
    //     }
    // }

    pub fn setup(&mut self) {
        // let event_ptr: *mut c_void = &mut self.events as *mut _ as *mut c_void;

        // // set the responder class delegate
        // use platform::platform::responder::*;
        // let responder: id = unsafe { msg_send![get_window_responder_class(), new] };
        
        // unsafe {
        //     msg_send![responder, setViewController: event_ptr];
        //     NSView::addSubview_(self.nsview, responder);
        //     msg_send![self.nswindow, setDelegate:responder];
        // }
        // println!("{:?}", (responder));

        // let window: id = unsafe { msg_send![window, window] };
        // let r: id = unsafe { msg_send![self.nswindow, delegate] };
        // println!("{:?}", (r, responder ));
        // unsafe { msg_send![responder, testHandler]; }

        // let e: &mut Box<WindowEvents> = unsafe { &mut *(event_ptr as *mut Box<WindowEvents>) };
        // println!("{:?}", (*e).title);
    }

    pub fn make_resizable(&mut self) {
        // make resizable
        unsafe { self.nswindow.setStyleMask_(self.nswindow.styleMask() | NSResizableWindowMask) };
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let nstitle = NSString::alloc(nil).init_str(title);
            self.nswindow.setTitle_(nstitle);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        unsafe { NSWindow::setBackgroundColor_(self.nswindow, color.nscolor()) };
    }

    pub fn run(&mut self) {
        // if let Some(ref mut on_load) = self.on_load_callback {
        //     on_load();
        // }
        unsafe {
            let app = NSApp();
            app.run();
        }
    }

    pub fn add_subview(&mut self, view: id) {
        unsafe { NSView::addSubview_(self.nsview, view) };
    }

    pub fn frame(&self) -> Rect {
        Rect::from_nsrect(unsafe { NSView::frame(self.nsview) })
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_nsrect(unsafe { NSView::bounds(self.nsview) })
    }

   // /// Attach a Window class to an existing window.
   // pub fn attach_to<EH:EventHandler>(host_nsview: *mut c_void, handler: &H) -> Result<Window, String> {
   //      let host_window = unsafe { msg_send![host_nsview as id, window] };
   //      // let host_nsview = unsafe { NSWindow::contentView(host_window) };

   //      let child_nsview = unsafe { NSView::alloc(nil) };
   //      let child_view = unsafe { child_nsview.initWithFrame_(NSView::frame(host_nsview as id)) };
   //      unsafe { NSView::addSubview_((host_nsview as id), child_view) }

   //      Ok(Window {
   //          events: Box::new(WindowEvents{
   //              on_file_drop_callback: None,
   //          }),
   //          nswindow: host_window,
   //          nsview: host_nsview as id,
   //          handler: handler,
   //      })
   // }

    pub fn on_file_drop(&mut self, callback: RefCell<Box<FnMut(String)>>) {
        self.events.on_file_drop_callback = Some(callback)
    }

   // unsafe fn prepare_for_display(&mut window: Window) {
   //     // self.view.set_wants_best_resolution_opengl_surface(YES);

   //     msg_send![self.nswindow, setAcceptsMouseMovedEvents: YES];
   //     msg_send![self.nswindow, makeKeyAndOrderFront: nil];
   //     msg_send![self.nswindow, setContentView: self.nsview];
   //     msg_send![self.nswindow, makeFirstResponder: self.nsview];
   //     msg_send![self.nswindow, center];

   //     // need to [NSApp activateIgnoringOtherApps:YES]; and find out what it does.
   // }

}
