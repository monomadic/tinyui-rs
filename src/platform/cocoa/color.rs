use cocoa::base::{ id, class };

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

    pub fn system_gray() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), systemGrayColor] } }
    }

    pub fn black() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), blackColor] } }
    }

    pub fn white() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), whiteColor] } }
    }

    pub fn clear() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), clearColor] } }
    }

    pub fn nscolor(&self) -> id {
        self.id
    }
}
