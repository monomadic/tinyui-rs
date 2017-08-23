use cocoa::foundation::{ NSRect, NSPoint, NSSize };

pub struct Rect {
    origin: Point,
    size: Size,
}

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Size {
    width: f64,
    height: f64,
}

impl Rect {

    pub fn new(x: f64, y: f64, w: f64, h:f64) -> Self {
        Rect{ origin: Point{ x: x, y: y }, size: Size{ width: w, height: h }}
    }

    pub fn to_nsrect(&self) -> NSRect {
        NSRect::new(NSPoint::new(self.origin.x, self.origin.y), NSSize::new(self.size.width, self.size.height))
    }

}