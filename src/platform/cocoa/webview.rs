#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa::base::{ id, nil, NO, YES, class };
use cocoa::foundation::{ NSString };
use cocoa::appkit::{ NSColor };

use objc::runtime::{ Class, Object, Protocol, Sel };
use objc::declare::{ ClassDecl };
use objc;

use std;

use Rect;
use Window;
use Color;
use EventHandler;

#[link(name = "WebKit", kind = "framework")]
extern {
    pub static WKScriptMessageHandler: id;
}

pub struct WebView {
    id: id,
}

fn nsstring_to_str(string: id) -> String {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![string, UTF8String];
        bytes as *const u8
    };
    let len = unsafe { string.len() };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len);
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

pub fn wk_script_message_handler_class() -> &'static Class {
    use std::sync::{Once, ONCE_INIT};

    static REGISTER_CUSTOM_SUBCLASS: Once = ONCE_INIT;
    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = Class::get("WKUserContentController").unwrap();
        let mut decl = ClassDecl::new("NotificationScriptMessageHandler", superclass).unwrap();

        extern fn userContentController(this: &Object, _cmd: Sel, didReceive: bool, message: id) {
            let name: &str = &nsstring_to_str(unsafe { msg_send![message, name] });
            let body: &str = &nsstring_to_str(unsafe { msg_send![message, body] });
            println!("{:?} {:?}", name, body);

            match name {
                "notification" => {
                    println!("notification...")
                },
                _ => {
                    println!("nothing...");
                }
            }
        }

        unsafe {
            decl.add_method(sel!(userContentController:didReceiveScriptMessage:),
                userContentController as extern fn(&Object, Sel, bool, id));
        }

        decl.register();
    });

    Class::get("NotificationScriptMessageHandler").expect("NotificationScriptMessageHandler to be valid.")
}

impl WebView {
    pub fn new(position: Rect) -> Self {
        unsafe {

            // Delegate

            // let protocols = Protocol::protocols();
            // for protocol in protocols.iter() {
            //     println!("{:?}", protocol.name());
            // }

            let cls = wk_script_message_handler_class();
            let controller = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            // WKUserContentController
            // let cls = Class::get("WKUserContentController").expect("WKUserContentController to exist");
            // let controller = {
            //     let obj: *mut Object = msg_send![cls, alloc];
            //     let obj: *mut Object = msg_send![obj, init];
            //     obj
            // };

            msg_send![controller, addScriptMessageHandler:controller name:NSString::alloc(nil).init_str("notification")];

            // WKWebViewConfiguration
            let cls = Class::get("WKWebViewConfiguration").expect("WKWebViewConfiguration to exist");
            let configuration = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            // (*configuration).set_ivar("userContentController", controller);
            // configuration.userContentController = controller;
            msg_send![configuration, setUserContentController:controller];

            // WKWebView
            let cls = Class::get("WKWebView").unwrap();
            let webview = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj,
                    initWithFrame: position.to_nsrect()
                    configuration: configuration ];
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
                let obj: *mut Object = msg_send![cls, fileURLWithPath:NSString::alloc(nil).init_str("file:///localhost/")];
                obj
            };
            msg_send![self.id,
                loadHTMLString:NSString::alloc(nil).init_str(html)
                baseURL:nsurl];
        }
    }

    pub fn attach<H:EventHandler>(&mut self, window: &mut Window<H>) {
        window.add_subview(self.id);
    }
}
