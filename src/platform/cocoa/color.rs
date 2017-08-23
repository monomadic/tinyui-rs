use cocoa::base::{ id, nil, class };

pub struct Color {
    id: id,
}

impl Color {
    pub fn red() -> Color {
        Color { id: unsafe{ msg_send![class("NSColor"), redColor] } }
    }

    pub fn nscolor(&self) -> id {
        self.id
    }
}
