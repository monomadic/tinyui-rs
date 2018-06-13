#[macro_use] extern crate objc;
extern crate cocoa;
extern crate core_graphics;


mod platform;
pub use platform::app::*;
pub use platform::window::*;
pub use platform::font::*;
pub use platform::label::*;
pub use platform::rect::*;
pub use platform::color::*;
pub use platform::button::*;
pub use platform::slider::*;
pub use platform::webview::*;

mod events;
pub use events::*;

mod error;
pub use error::*;
