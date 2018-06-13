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

    // split Rectangle into equal sized left & right rectangles
    pub fn split_vertical(self) -> (Rect, Rect) {

        let left_rect = Rect {
            origin: Point { x: self.origin.x, y: self.origin.y },
            size: Size { width: self.size.width / 2., height: self.size.height }
        };
        let right_rect = Rect {
            origin: Point { x: self.origin.x + self.size.width / 2., y: self.origin.y },
            size: Size { width: self.size.width / 2., height: self.size.height }
        };
        (
            left_rect,
            right_rect
        )
    }

    // split Rect into equal sized top & bottom rectangles
    // Origin of Rects are bottom left (Screen dimensions) so
    // bottom is with lower Y coordinates
    pub fn split_horizontal(self) -> (Rect, Rect) {
        let bottom_rect = Rect {
            origin: Point { x: self.origin.x, y: self.origin.y },
            size: Size { width: self.size.width, height: self.size.height / 2. }
        };
        let top_rect = Rect {
            origin: Point { x: self.origin.x, y: self.origin.y + self.size.height / 2.0 },
            size: Size { width: self.size.width, height: self.size.height / 2. }
        };
        (
            top_rect,
            bottom_rect
        )
    }

    pub fn inset(self, pts: f64) -> Rect {
        Rect {
            origin: Point{ x: self.origin.x + pts, y: self.origin.y + pts },
            size: Size { width: self.size.width - (pts*2.), height: self.size.height - (pts*2.) },
        }
    }
}
