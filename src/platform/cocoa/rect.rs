use cocoa::foundation::{ NSRect, NSPoint, NSSize };

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h:f64) -> Self {
        Rect{ origin: Point{ x: x, y: y }, size: Size{ width: w, height: h }}
    }

    pub fn from_nsrect(rect: NSRect) -> Self {
        Rect{ origin: Point{ x: rect.origin.x, y: rect.origin.y }, size: Size{ width: rect.size.width, height: rect.size.height }}
    }

    pub fn to_nsrect(&self) -> NSRect {
        NSRect::new(NSPoint::new(self.origin.x, self.origin.y), NSSize::new(self.size.width, self.size.height))
    }
}
