#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO, YES, class };
use cocoa::foundation::{ NSString };
use cocoa::appkit::{ NSColor };
use objc::runtime::{ Class, Object };
use objc;

use Rect;
use Window;
use Color;

#[link(name = "WebKit", kind = "framework")]
extern {}

pub struct WebView {
    id: id,
}

impl WebView {
    pub fn new(position: Rect) -> Self {
        unsafe {
            let cls = Class::get("WKWebView").unwrap();
            let webview = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, initWithFrame: position.to_nsrect()];
                obj
            };

            msg_send![webview, setOpaque:NO];
            msg_send![webview, setBackgroundColor:Color::clear().nscolor()];

            WebView {
                id: webview,
            }
        }
    }

    pub fn set_size(&mut self, html: &str) {
        unsafe { msg_send![self.id, isLoading] }
    }

    pub fn is_loading(&mut self, html: &str) -> bool {
        unsafe { msg_send![self.id, isLoading] }
    }

    pub fn load_html_string(&mut self, html: &str) {
        unsafe {
            let cls = Class::get("NSURL").unwrap();
            let nsurl = {
                let obj: *mut Object = msg_send![cls, fileURLWithPath:NSString::alloc(nil).init_str("file:///nowhere/")];
                obj
            };
            msg_send![self.id,
                loadHTMLString:NSString::alloc(nil).init_str(html)
                baseURL:nsurl];
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
