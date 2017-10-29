#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa;
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
use { EventHandler, Event };
use platform::platform::responder::send_event;

#[link(name = "WebKit", kind = "framework")]
extern {
    pub static WKScriptMessageHandler: id;
}

#[derive(Copy, Clone)]
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

        extern fn userContentController(this: &mut Object, _cmd: Sel, didReceive: bool, message: id) {
            let name = nsstring_to_str(unsafe { msg_send![message, name] });
            let body = nsstring_to_str(unsafe { msg_send![message, body] });

            let webview = unsafe { msg_send![message, webView] };
            send_event(webview, Event::WebEvent(name, body));
        }

        unsafe {
            decl.add_method(sel!(userContentController:didReceiveScriptMessage:),
                userContentController as extern fn(&mut Object, Sel, bool, id));
        }

        decl.register();
    });

    Class::get("NotificationScriptMessageHandler").expect("NotificationScriptMessageHandler to be valid.")
}

pub fn navigation_delegate_class() -> &'static Class {
    use std::sync::{Once, ONCE_INIT};

    static REGISTER_CUSTOM_SUBCLASS: Once = ONCE_INIT;
    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = Class::get("WKWebView").expect("WKWebView to be available");
        let mut decl = ClassDecl::new("NavigationDelegate", superclass).expect("WKWebView to be subclassable");

        decl.add_protocol(Protocol::get("WKNavigationDelegate").expect("WKNavigationDelegate protocol to exist"));

        extern fn didCommitNavigation(this: &Object, _cmd: Sel, webview: id, navigation: id) {
            send_event(webview, Event::WebViewContentRecieved);
        }
        extern fn didFinishNavigation(this: &Object, _cmd: Sel, webview: id, navigation: id) {
            send_event(webview, Event::WebViewFinishedLoading);
        }

        unsafe {
            decl.add_method(sel!(webView:didCommitNavigation:),
                didCommitNavigation as extern fn(&Object, Sel, id, id));
            decl.add_method(sel!(webView:didFinishNavigation:),
                didFinishNavigation as extern fn(&Object, Sel, id, id));
        }

        decl.register();
    });

    Class::get("NavigationDelegate").expect("NavigationDelegate to be valid.")
}

impl WebView {
    pub fn new(position: Rect) -> Self {
        unsafe {

            // WKUserContentController
            let cls = wk_script_message_handler_class();
            let scripthandler = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            msg_send![scripthandler, addScriptMessageHandler:scripthandler name:NSString::alloc(nil).init_str("notification")];

            // WKWebViewConfiguration
            let cls = Class::get("WKWebViewConfiguration").expect("WKWebViewConfiguration to exist");
            let configuration = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            // configuration.userContentController = scripthandler;
            msg_send![configuration, setUserContentController:scripthandler];

            // WKWebView
            let cls = Class::get("WKWebView").expect("WKWebView to exist");
            let webview = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj,
                    initWithFrame: position.to_nsrect()
                    configuration: configuration ];
                obj
            };

            // WKNavigationDelegate
            let cls = navigation_delegate_class();
            let navigation_delegate = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };
            msg_send![webview, setNavigationDelegate:navigation_delegate];

            // make window transparent
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
                let obj: *mut Object = msg_send![cls, fileURLWithPath:NSString::alloc(nil).init_str("")];
                obj
            };

            msg_send![self.id,
                loadHTMLString:NSString::alloc(nil).init_str(html)
                baseURL:nsurl];
                
            msg_send![self.id, setOpaque:NO];
            msg_send![self.id, setBackgroundColor:Color::clear().nscolor()];
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
