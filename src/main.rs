extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::traits::{Drawable};
use sfml::graphics::{RenderWindow, Color, Texture, IntRect, RenderTarget, Font, Text};
use sfml::graphics::rc::{Sprite};

use sfml::window::{VideoMode, ContextSettings, event, WindowStyle};
use sfml::window::keyboard::Key;
use sfml::system::Clock;

mod renderables;
use renderables::Renderable;

fn create_terrain() -> renderables::Terrain {
    let terrain_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/terrain_tileset.png").unwrap()));

    let mut terrain_sprite: Sprite = match Sprite::new() {
        Some(sprite)  => sprite,
        None          => panic!("Cannot found resource: image")
    };

    terrain_sprite.set_texture(terrain_texture, false);
    terrain_sprite.set_position2f(0f32, 0f32);

    renderables::Terrain {
        sprite: terrain_sprite,
        offset: (210, 16),
        tile_size: 30,
    }
}

fn create_frog_sprite(x: f32, y: f32, animation_index: i32) -> renderables::AnimatedSprite {
    let mut frog_sprite = Sprite::new().expect("Cannot create a new Sprite");

    let frog_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/3571.png").unwrap()));

    frog_sprite.set_texture(frog_texture, false);

    renderables::AnimatedSprite {
        sprite: frog_sprite,
        offset: (4, animation_index * 28),
        position: (x, y),
        width: 19,
        height: 30,
        frames: 7,
        last_frame_at: 0
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
    let mut frog_sprite1 = create_frog_sprite(400f32, 100f32, 0);
    let mut frog_sprite2 = create_frog_sprite(200f32, 50f32, 1);
    let mut frog_sprite3 = create_frog_sprite(100f32, 400f32, 2);
    let mut frog_sprite4 = create_frog_sprite(350f32, 100f32, 3);

    let mut terrain = create_terrain();
    let mut stats = renderables::FrameStats::new();

    let mut clock = Clock::new();

    while window.is_open() {
        handle_window_events(&mut window);
        window.clear(&clear_color);

        let elapsed = clock.restart().as_milliseconds();

        terrain.render(&mut window, elapsed);
        frog_sprite1.render(&mut window, elapsed);
        frog_sprite2.render(&mut window, elapsed);
        frog_sprite3.render(&mut window, elapsed);
        frog_sprite4.render(&mut window, elapsed);
        stats.render(&mut window, elapsed);

        window.display();
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
