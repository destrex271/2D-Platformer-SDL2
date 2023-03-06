use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub enum PlayerMovements {
    Jump,
    Crouch,
    Forward,
    Backward,
}

pub struct Player {
    sprite: Rect,
    height: u32,
    width: u32,
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
}

impl Player {
    pub fn new(x: i32, y: i32, height: u32, width: u32) -> Player {
        Player {
            sprite: Rect::new(x, y, width, height),
            x,
            y,
            height,
            width,
            speed_x: 2,
            speed_y: 2,
        }
    }

    // Getter functions
    pub fn get_sprite(&self) -> Rect {
        self.sprite
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_speed_x(&self) -> i32 {
        self.speed_x
    }

    pub fn get_speed_y(&self) -> i32 {
        self.speed_y
    }

    pub fn get_player_color(&self) -> Color {
        Color::RGB(255, 255, 255)
    }

    pub fn get_player_x(&self) -> i32 {
        self.x
    }

    pub fn get_player_y(&self) -> i32 {
        self.y
    }

    // Setter functions

    pub fn update_player_state(&mut self) {
        self.sprite.set_x(self.x);
        self.sprite.set_y(self.y);
    }

    pub fn move_player(&mut self, dirn: PlayerMovements) {
        match dirn {
            PlayerMovements::Jump => {}
            PlayerMovements::Crouch => {}
            PlayerMovements::Forward => {
                self.x += self.speed_x;
            }
            PlayerMovements::Backward => {
                self.x -= self.speed_x;
            }
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        println!("Player Destroyed!");
    }
}
