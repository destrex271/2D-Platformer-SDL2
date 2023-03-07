use sdl2::rect::Rect;

pub struct Ground {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

impl Ground {
    pub fn new(width: u32, height: u32, x: i32, y: i32) -> Ground {
        Ground {
            width,
            height,
            x,
            y,
        }
    }
    pub fn get_sprite(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
