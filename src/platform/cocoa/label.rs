use cocoa::base::{ id, nil, NO };
use cocoa::appkit::NSTextField;
use cocoa::foundation::{ NSString };
use Color;
use Font;
use Rect;
use Window;

#[derive(Copy, Clone)]
pub struct Label {
    id: id,
}

// impl Drop for Label {
//     fn drop(&mut self) {
//         unsafe { msg_send![self.id, removeFromSuperview] };
//     }
// }

impl Label {
    pub fn new(text: &str, position: Rect) -> Self {
        unsafe {
            let label = NSTextField::alloc(nil).initWithFrame_(position.to_nsrect());
            label.setStringValue_(NSString::alloc(nil).init_str(text));

            msg_send![label, setBezeled:NO];
            msg_send![label, setDrawsBackground:NO];
            msg_send![label, setEditable:NO];
            msg_send![label, setSelectable:NO];
            msg_send![label, setStringValue:NSString::alloc(nil).init_str(text)];

            Label { id: label }
        }
    }

    pub fn set_text(&self, text: &str) {
        unsafe { self.id.setStringValue_(NSString::alloc(nil).init_str(text)) };
    }

    pub fn set_text_color(&self, color: Color) {
        unsafe { msg_send!(self.id, setTextColor:color.nscolor() ) }
    }

    /// Set the font of a label
    ///
    /// # Arguments
    ///
    /// * `font` - A Font object specifiyng the font
    ///
    /// # Example
    ///
    /// ```
    /// app.label.set_font(tinyui::Font::init("Helvetica Neue Medium", 18.));
    /// app.slider_label.set_font(tinyui::Font::message_font(0.)); // the default font size
    /// ```
    ///
    pub fn set_font(&self, font: Font) {
        unsafe { msg_send!(self.id, setFont:font.nsfont() ) }
    }



    pub fn attach(&mut self, window: &mut Window) {
        window.add_subview(self.id);
    }
}
