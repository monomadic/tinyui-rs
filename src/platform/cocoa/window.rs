use cocoa::base::{ id, nil };
use std::os::raw::c_void;

use cocoa::base::{ NO, YES };
use cocoa::foundation::{ NSString, NSRect, NSSize, NSPoint, NSAutoreleasePool };
use cocoa::appkit::{ NSApp, NSApplication, NSWindow, NSTitledWindowMask, NSBackingStoreBuffered, NSRunningApplication,
                     NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular };

pub struct Window<'cb> {
    nswindow: id,
    nsview: id,
    on_load_callback: Option<Box<FnMut() + 'cb>>,
}

//impl Drop for Window {
//
//}

impl<'cb> Window<'cb> {

    /// Create a new Window from scratch.
    pub fn new() -> Result<Window<'cb>, String> {

        // callback();

        let window = unsafe { NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(200., 200.)),
                                                          NSTitledWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO) };

        unsafe {
            //            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            app.activateIgnoringOtherApps_(YES);

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            window.center();
            window.setOpaque_(YES);
            window.makeKeyAndOrderFront_(nil);
        }

        Ok(Window {
            nswindow: window,
            nsview: unsafe { NSWindow::contentView(window) },
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

    pub fn run(&mut self) {
        // if let Some(ref mut on_load) = self.on_load_callback {
        //     on_load();
        // }
        unsafe {
            let app = NSApp();
            app.run();
        }
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
