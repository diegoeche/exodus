extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use sfml::traits::{Drawable};
use sfml::graphics::{RenderWindow, Color, Texture, IntRect, RenderTarget, Font, Text};
use sfml::graphics::rc::{Sprite};
use sfml::window::{VideoMode, ContextSettings, WindowStyle};
use sfml::window::event;
use sfml::window::keyboard::Key;
use sfml::window::mouse::MouseButton;

use sfml::system::Clock;
mod renderables;
use renderables::Renderable;

struct World {
    objects: HashMap<(u32, u32), Object>,
    terrain_sprites: HashMap<(u32, u32), Sprite>,
    player: renderables::Unit,
    default_sprite: Sprite,
    map_size: (u32, u32),
    tile_size:  u32
}

impl Renderable for World {
    fn render(&mut self, window: &mut RenderWindow, elapsed: i32) {
        let (x_size, y_size) = self.map_size;
        for i in (0..x_size) {
            for j in (0..y_size) {
                let new_x = (i * self.tile_size as u32) as f32;
                let new_y = (j * self.tile_size as u32) as f32;
                self.default_sprite.set_position2f(new_x, new_y);
                window.draw(&(self.default_sprite));

                match self.objects.get(&(i, j)) {
                    Some(_) => {
                        self.player.render(window, elapsed);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn create_frog_sprite(
    (x, y): (f32, f32),
    animation_index: i32,
    texture: Rc<RefCell<Texture>>) -> renderables::AnimatedSprite {
    let mut frog_sprite = Sprite::new().expect("Cannot create a new Sprite");

    frog_sprite.set_texture(texture, false);

    renderables::AnimatedSprite {
        sprite: frog_sprite,
        offset: (4, animation_index * 28),
        position: (x, y),
        width: 19,
        height: 30,
        frames: 7,
        last_frame_at: 0,
    }
}

fn main() {
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
        "SFML borrow ressources Example", WindowStyle::Close, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    let clear_color = Color::black();

    let frog_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/3571.png").unwrap()));


        let down  = create_frog_sprite((0f32, 0f32), 0, frog_texture.clone());
        let up    = create_frog_sprite((0f32, 0f32), 1, frog_texture.clone());
        let left  = create_frog_sprite((0f32, 0f32), 2, frog_texture.clone());
        let right = create_frog_sprite((0f32, 0f32), 3, frog_texture.clone());

        let mut entity = renderables::Unit::new(
            up,
            down,
            left,
            right,
        );


    let mut stats = renderables::FrameStats::new();
    let mut objects = HashMap::new();
    objects.insert((0, 4), Object::Player);

    let terrain_texture: Rc<RefCell<Texture>> =
        Rc::new(RefCell::new(Texture::new_from_file("resources/54728-none.png").unwrap()));

    let mut grass_sprite: Sprite = Sprite::new().unwrap();
    grass_sprite.set_texture(terrain_texture, false);
    let frame = IntRect::new(60, 0, 30, 30);
    grass_sprite.set_texture_rect(&frame);
    grass_sprite.set_position2f(100f32, 100f32);

    let mut terrain_sprites = HashMap::new();

    let mut world = World {
        objects: objects,
        terrain_sprites: terrain_sprites,
        player: entity,
        map_size: (15, 15),
        tile_size: 30,
        default_sprite: grass_sprite
    };

    // let mut terrain = create_terrain();
    let mut clock = Clock::new();

    while window.is_open() {
        handle_window_events(&mut window, &mut world.player);
        window.clear(&clear_color);

        let elapsed = clock.restart().as_milliseconds();

        world.render(&mut window, elapsed);
        // world.player.render(&mut window, elapsed);
        // window.draw(&(world.grass));
        // terrain.render(&mut window, elapsed);
        stats.render(&mut window, elapsed);
        window.display();
    }
}

fn handle_window_events(window: &mut RenderWindow, entity: &mut renderables::Unit) {
    for event in window.events() {
        match event {
            event::Closed => window.close(),
            event::MouseButtonPressed{button, ..} => {
            },
            event::KeyPressed{code, ..} => match code {
                Key::Escape => {
                    window.close();
                    break;
                },
                key  => {
                    entity.consume_input(key);
                }
            },
            _ => {}
        }
    }
}

// Voxel Representation
enum Voxel {
    // Maybe better "Symmetrical"
    Empty {
        id: i32
    },
}

enum Object {
    Player,
}
