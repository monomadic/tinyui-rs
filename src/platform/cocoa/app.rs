use cocoa::appkit::{ NSApp, NSApplication };
use cocoa::base::{ nil };

pub struct App {
}

impl App {
    pub fn quit() {
        unsafe {
            let app = NSApp();
            msg_send![app, terminate:nil];
        }
    }

    pub fn run() {
        unsafe {
            let app = NSApp();
            app.run();
        }
    }
}
