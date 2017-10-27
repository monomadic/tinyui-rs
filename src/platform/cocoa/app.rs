use cocoa::appkit::{ NSApp, NSApplication };
use cocoa::base::id;

pub struct App {
    id: id,
}

impl App {
    pub fn run() -> App {
        unsafe {
            let app = NSApp();
            app.run();
            App { id: app }
        }
    }

    pub fn terminate(&self) {
        unsafe { msg_send![self.id, terminate] };
    }
}
