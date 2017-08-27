#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO, class };
use objc::runtime::{ Class, Object };
use objc;

use Rect;
use Window;

#[link(name = "WebKit", kind = "framework")]
extern {
    // pub static WKWebView: id;
}

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

            // set delegate
            // msg_send![webview, navigationDelegate:webview];


            // let cls = Class::get("WKWebView").unwrap();
            // msg_send![webview, navigationDelegate:webview];

            WebView {
                id: webview,
            }
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
