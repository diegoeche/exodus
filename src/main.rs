extern crate sfml;

// use std::io;
// use std::io::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::traits::{Drawable};
use sfml::graphics::{RenderWindow, Color, Texture, Font, IntRect, RenderTarget};
use sfml::graphics::rc::{Sprite};

use sfml::window::{VideoMode, ContextSettings, event, WindowStyle};
use sfml::window::keyboard::Key;
// use sfml::system::Vector2f;
use sfml::system::Clock;

struct AnimatedSprite {
    sprite: Sprite,
    width: i32,
    height: i32,
    last_frame_at: i32,
    frames: i32,
}

fn create_terrain() -> Sprite {
    let terrain_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/terrain_tileset.png").unwrap()));

    let mut terrain_sprite: Sprite = match Sprite::new() {
        Some(sprite)  => sprite,
        None          => panic!("Cannot found resource: image")
    };

    terrain_sprite.set_texture(terrain_texture, false);
    terrain_sprite.set_position2f(0f32, 0f32);

    terrain_sprite
}

fn create_frog_sprite() -> AnimatedSprite {
    let mut frog_sprite = Sprite::new().expect("Cannot create a new Sprite");

    let frog_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/3571.png").unwrap()));

    frog_sprite.set_texture(frog_texture, false);
    frog_sprite.set_position2f(400f32, 300f32);

    AnimatedSprite {
        sprite: frog_sprite,
        width: 20,
        height: 30,
        frames: 7,
        last_frame_at: 0
    }
}


impl AnimatedSprite {
    fn render(&mut self, window: &mut RenderWindow, time: i32) {
        self.last_frame_at = self.last_frame_at + time;
        let index = (self.last_frame_at + time) / 100;
        let offset = ((index % self.frames) * self.width) + 2;
        let frame = IntRect::new(offset, 0, self.width, self.height);
        self.sprite.set_texture_rect(&frame);
        window.draw(&(self.sprite));
    }
}

fn handle_window_events(window: &mut RenderWindow) {
    for event in window.events() {
        match event {
            event::Closed => window.close(),
            event::KeyPressed{code, ..} => match code {
                Key::Escape => {
                    window.close();
                    break;
                },
                _ => {
                    // Do something!
                }
            },
            _ => {}
        }
    }
}

fn main() {
    // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
        "SFML borrow ressources Example", WindowStyle::Close, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    let clear_color = Color::black();

    // Create a Sprite_Texture
    let mut frog_sprite = create_frog_sprite();
    let terrain = create_terrain();

    let mut clock = Clock::new();

    while window.is_open() {
        handle_window_events(&mut window);
        window.clear(&clear_color);

        let elapsed = clock.restart().as_milliseconds();
        frog_sprite.render(&mut window, elapsed);
        window.display();
    }
}
