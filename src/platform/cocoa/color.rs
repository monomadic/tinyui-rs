use cocoa::base::{ id, nil, class };

pub struct Color {
    id: id,
}

impl Color {
    pub fn red() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), redColor] } }
    }

    pub fn green() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), greenColor] } }
    }

    pub fn dark_black() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), charcoalColor] } }
    }

    pub fn black() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), blackColor] } }
    }

    pub fn clear() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), clearColor] } }
    }

    pub fn nscolor(&self) -> id {
        self.id
    }
}
