use cocoa::appkit::{ NSApp, NSApplication };

pub struct App {
}

impl App {
    pub fn run() {
        unsafe {
            let app = NSApp();
            app.run();
        }
    }
}
