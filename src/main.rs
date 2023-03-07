// Module and crate declarations
extern crate sdl2;
pub mod game_config;
pub mod ground;
pub mod player;

// System Libraries
use std::time::Duration;

// sdl2 libraries
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

// Game config lib
use game_config::{GameConfig, GameStatus};
use ground::Ground;
use player::{Player, PlayerMovements};

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 700;

fn main() {
    let mut game_config = GameConfig::new(GameStatus::Playing, 0);
    let mut player = Player::new(
        100,
        HEIGHT as i32 - 100,
        20,
        20,
        WIDTH as i32,
        HEIGHT as i32,
    );
    let ground = Ground::new(
        WIDTH,
        100,
        0,
        HEIGHT as i32 - 100 + player.get_height() as i32,
    );

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut _image_ctx = sdl2::image::init(InitFlag::PNG);

    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let sky_texture_creator = canvas.texture_creator();
    let ground_texture_creator = canvas.texture_creator();

    let sky_texture = sky_texture_creator
        .load_texture("/home/akshat/fun/rust/game/src/assets/sky.png")
        .unwrap();
    let ground_texture = ground_texture_creator
        .load_texture("/home/akshat/fun/rust/game/src/assets/rocks_1.png")
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game_config.quit(),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.move_player(PlayerMovements::Forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => player.move_player(PlayerMovements::Backward),
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
        // Rendering sky
        canvas.copy(&sky_texture, None, None).unwrap();

        // Rendering Ground
        canvas.set_draw_color(Color::RGB(255, 0, 100));
        canvas.fill_rect(ground.get_sprite()).unwrap();

        // Rendering Player
        // player.move_player(PlayerMovements::Forward);
        canvas.set_draw_color(player.get_player_color());
        canvas.fill_rect(player.get_sprite()).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
