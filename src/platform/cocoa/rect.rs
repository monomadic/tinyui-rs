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
        Rect{ origin: Point{ x: rect.origin.y, y: rect.origin.x }, size: Size{ width: rect.size.height, height: rect.size.width }}
    }

    pub fn to_nsrect(&self) -> NSRect {
        NSRect::new(NSPoint::new(self.origin.y, self.origin.x), NSSize::new(self.size.height, self.size.width))
    }

    pub fn split_vertical(self) -> (Rect, Rect) {
        (
            Rect{
                origin: Point{ x: self.origin.x, y: self.origin.y },
                size: Size { width: self.size.width, height: self.size.height / 2. }},
            Rect{
                origin: Point { x: self.origin.x, y: self.size.height / 2. },
                size: Size { width: self.size.width, height: self.size.height / 2. }}
        )
    }

    pub fn split_horizontal(self) -> (Rect, Rect) {
        (
            Rect{
                origin: Point{ x: self.origin.x, y: self.origin.y },
                size: Size { width: self.size.width / 2., height: self.size.height }},
            Rect{
                origin: Point { x: self.size.width / 2., y: self.origin.y },
                size: Size { width: self.size.width / 2., height: self.size.height }}
        )
    }

    pub fn inset(self, pts: f64) -> Rect {
        Rect {
            origin: Point{ x: self.origin.x + pts, y: self.origin.y + pts },
            size: Size { width: self.size.width - (pts*2.), height: self.size.height - (pts*2.) },
        }
    }
}
