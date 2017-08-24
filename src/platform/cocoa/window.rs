use cocoa::base::{ id, nil };
use std::os::raw::c_void;

use cocoa::base::{ NO, YES };
use cocoa::foundation::{ NSString, NSRect, NSSize, NSPoint, NSAutoreleasePool };
use cocoa::appkit::{ NSApp, NSApplication, NSWindow, NSView, NSTitledWindowMask, NSBackingStoreBuffered, NSRunningApplication,
                     NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular, NSFilenamesPboardType };

use Color;
use Rect;

pub struct Window<'cb> {
    nswindow: id,
    nsview: id,
    on_load_callback: Option<Box<FnMut() + 'cb>>,
}

//impl Drop for Window {
//
//}

use ViewController;

impl<'cb> Window<'cb> {

    /// Create a new Window from scratch.
    pub fn new<T:ViewController>(width: f64, height: f64, controller: *mut T) -> Result<Window<'cb>, String> {

        // callback();

        let window = unsafe { NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(width, height)),
                                                          NSTitledWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO) };

        let view = unsafe { NSWindow::contentView(window) };

        // set the responder class delegate
        use platform::platform::responder::*;
        let responder = unsafe { msg_send![get_window_responder_class(), new] };

        unsafe {
            //            let _pool = NSAutoreleasePool::new(nil);

            window.setAcceptsMouseMovedEvents_(YES); // msg_send![window, setAcceptsMouseMovedEvents: YES];
            window.makeKeyAndOrderFront_(nil);
            // window.setContentView_(view);
            window.makeFirstResponder_(view);
            
            NSView::addSubview_(view, responder);

            msg_send![window, setDelegate:responder];

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

        unsafe {
            msg_send![responder, setViewController: controller as *mut c_void];
        }

        Ok(Window {
            nswindow: window,
            nsview: view,
            on_load_callback: None,
        })
    }

    pub fn on_load(&mut self, callback: &'cb Fn()) {
        self.on_load_callback = Some(Box::new(callback))
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
        Rect::from_nsrect(unsafe { NSWindow::frame(self.nswindow) })
    }

//    /// Attach a Window class to an existing window.
//    pub fn attach(host_nsview: *mut c_void) -> Result<Window, String> {
//        let host_window = unsafe { msg_send![host_nsview as id, window] };
//
//        Ok(Window {
//            nswindow: host_window,
//            nsview: host_nsview as id,
//        })
//    }

//    unsafe fn prepare_for_display(&mut window: Window) {
//        // self.view.set_wants_best_resolution_opengl_surface(YES);
//
//        msg_send![self.nswindow, setAcceptsMouseMovedEvents: YES];
//        msg_send![self.nswindow, makeKeyAndOrderFront: nil];
//        msg_send![self.nswindow, setContentView: self.nsview];
//        msg_send![self.nswindow, makeFirstResponder: self.nsview];
//        msg_send![self.nswindow, center];
//
//        // need to [NSApp activateIgnoringOtherApps:YES]; and find out what it does.
//    }

}
