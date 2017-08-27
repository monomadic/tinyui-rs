#[macro_use] extern crate objc;
extern crate cocoa;

mod platform;
pub use platform::window::Window;
pub use platform::label::*;
pub use platform::rect::*;
pub use platform::color::*;
pub use platform::button::*;
pub use platform::webview::*;
