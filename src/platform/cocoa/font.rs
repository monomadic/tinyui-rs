extern crate core_graphics;

use cocoa::base::{id, class, nil};
use cocoa::foundation::NSString;
use core_graphics::base::CGFloat;

#[derive(Debug, Copy, Clone)]
pub struct Font {
    id: id,
}

/// Wraps some of the NSFont methods
///
/// See https://developer.apple.com/documentation/appkit/nsfont?changes=_1&language=objc
/// for more methods
///
impl Font {
    /// Returns a font from the given name, with a specific size
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the font. The font names on OSX can
    ///            be seen by running the *FontBook* app
    /// * `size` - a f64 that specifies the point size. To get the default size, pass a 0 or
    ///            a negative number
    ///
    /// # Example
    ///
    /// ```
    /// let font = Font::init("Helvetica Neue Bold", 18.);
    /// ```
    pub fn init(name: &str, size: f64) -> Font {
        unsafe {
            let nsname = NSString::alloc(nil).init_str(name);
            Font { id: msg_send![class("NSFont"), fontWithName: nsname size: size as CGFloat] }
        }
    }

    /// Returns the system font in the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - a f64 that specifies the point size. To get the default size, pass a 0 or
    ///            a negative number
    ///
    pub fn system_font(size: f64) -> Font { Font { id: unsafe { msg_send![class("NSFont"), systemFontOfSize:size as CGFloat] } } }

    /// Returns the bold system font in the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - a f64 that specifies the point size. To get the default size, pass a 0 or
    ///            a negative number
    ///
    pub fn bold_system_font(size: f64) -> Font { Font { id: unsafe { msg_send![class("NSFont"), boldSystemFontOfSize:size as CGFloat] } } }

    /// Returns the label font in the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - a f64 that specifies the point size. To get the default size, pass a 0 or
    ///            a negative number
    ///
    pub fn label_font(size: f64) -> Font { Font { id: unsafe { msg_send![class("NSFont"), labelFontOfSize:size as CGFloat] } } }

    /// Returns the message font in the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - a f64 that specifies the point size. To get the default size, pass a 0 or
    ///            a negative number
    ///
    pub fn message_font(size: f64) -> Font { Font { id: unsafe { msg_send![class("NSFont"), messageFontOfSize:size as CGFloat] } } }

    pub fn nsfont(&self) -> id {
        self.id
    }
}
