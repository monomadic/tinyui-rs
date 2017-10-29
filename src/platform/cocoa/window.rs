#![allow(dead_code)]

use cocoa::base::{ id, nil, NO, YES };
use cocoa::foundation::{ NSString, NSRect, NSSize, NSPoint };
use cocoa::appkit::{ NSApp, NSApplication, NSWindow, NSView, NSTitledWindowMask, NSClosableWindowMask, NSResizableWindowMask, NSBackingStoreBuffered, NSRunningApplication,
                     NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular, NSFilenamesPboardType };

use Color;
use Rect;

#[derive(Copy, Clone)]
pub struct Window {
    pub nswindow: id,
    pub nsview: id,
}

use Event;
pub trait EventHandler {
    fn handle(&mut self, event: Event);
}

pub struct Handler {
    pub handler: Box<EventHandler>,
}

impl EventHandler for Handler {
    fn handle(&mut self, event: Event) {
        self.handler.handle(event);
    }
}

// pub struct WindowEvents {
//     pub on_file_drop_callback: Option<RefCell<Box<FnMut(String)>>>,
// }

// impl Drop for Window {
//     fn drop(&mut self) {
//         unsafe { self.nsview.removeFromSuperview() };
//     }
// }

// impl WindowEvents {
//     pub fn on_mouse_down(&mut self) {
//         println!("Yaaaas!!");
//     }

//     pub fn on_file_drop(&mut self, path: String) {
//         if let Some(ref mut callback) = self.on_file_drop_callback {
//             let ref mut file_drop = *(callback.borrow_mut());
//             file_drop(path);
//         }
//     }
// }

pub enum WindowStyle {
    Default,
}

use Size;
pub struct WindowBuilder {
    pub title: &'static str,
    pub style: WindowStyle,
    pub size: Size,
}

impl WindowBuilder {
    pub fn build(&self) -> Window {
        Window::new(self.title, self.size.width, self.size.height).expect("window to build")
    }
}

impl Window {

    pub fn close(&mut self) {
        unsafe {
            self.nsview.removeFromSuperview();
        };

        unsafe {
            msg_send![self.nsview, release];
            msg_send![self.nswindow, release];
        };


        self.nsview = nil;
        self.nswindow = nil;
    }

    pub fn set_handler<H:'static + EventHandler>(self, handler: H) {
        use platform::platform::responder::*;
        let responder: id = unsafe { msg_send![self.nswindow, delegate] };
        set_event_handler_contained(responder, Handler{ handler: Box::new(handler) });
    }

    /// Create a new Window from scratch.
    pub fn new(title: &str, width: f64, height: f64) -> Result<Window, String> {

        // set the responder class delegate
        use platform::platform::responder::*;
        let responder: id = unsafe { msg_send![get_window_responder_class(), new] };

        let window = unsafe { NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(width, height)),
                                                          NSTitledWindowMask | NSClosableWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO) };

        let view = unsafe { NSWindow::contentView(window) };

        unsafe {
            msg_send![window, setDelegate:responder];

            window.setAcceptsMouseMovedEvents_(YES);
            window.makeKeyAndOrderFront_(nil);
            window.makeFirstResponder_(view);

            let nstitle = NSString::alloc(nil).init_str(title);
            window.setTitle_(nstitle);

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
        })
    }

    pub fn make_resizable(&mut self) {
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
}
