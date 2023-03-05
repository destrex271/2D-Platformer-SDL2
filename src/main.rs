// Module and crate declarations
extern crate sdl2;
pub mod game_config;

// System Libraries
use std::time::Duration;

// sdl2 libraries
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

// Game config lib
use game_config::{GameConfig, GameStatus};

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 700;

fn main() {
    let mut game_config = GameConfig::new(GameStatus::Playing, 0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'main: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game_config.quit(),
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => game_config.pause(),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => game_config.resume(),
                _ => {}
            }
        }

        // Checking current game_config
        match game_config.get_status() {
            GameStatus::Quit => {
                // Quit Action Message on Screen
                println!("Quiting game!");
                break 'main;
            }
            GameStatus::Paused => {
                println!("Game Paused!");
                continue;
            }
            _ => {}
        }

        println!("Playing");
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
