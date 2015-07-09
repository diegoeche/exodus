extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;

use sfml::traits::{Drawable};
use sfml::graphics::{RenderWindow, IntRect, RenderTarget, Font};
use sfml::graphics::rc::{Sprite, Text};
use sfml::window::keyboard::Key;

pub struct Terrain {
    pub sprite: Sprite,
    pub offset: (i32, i32),
    pub tile_size: i32,
}

pub trait Renderable {
    fn render(&mut self, window: &mut RenderWindow, elapsed: i32);
}

impl Renderable for Terrain {
    fn render(&mut self, window: &mut RenderWindow, _: i32) {
        let (x_offset, y_offset) = self.offset;
        let frame = IntRect::new(x_offset, y_offset, self.tile_size, self.tile_size);
        self.sprite.set_texture_rect(&frame);
        let size = window.get_size();

        let tiles_in_x = ((size.x / (self.tile_size as u32))) + 1;
        let tiles_in_y = ((size.y / (self.tile_size as u32))) + 1;

        for i in (0..tiles_in_x) {
            for j in (0..tiles_in_y) {
                let new_x = (i * self.tile_size as u32) as f32;
                let new_y = (j * self.tile_size as u32) as f32;
                self.sprite.set_position2f(new_x, new_y);
                window.draw(&(self.sprite));
            }
        }
    }
}

#[derive(Clone)]
pub struct AnimatedSprite {
    pub sprite: Sprite,
    pub offset: (i32, i32),
    pub position: (f32, f32),
    pub width: i32,
    pub height: i32,
    pub last_frame_at: i32,
    pub frames: i32,
}

impl Renderable for AnimatedSprite {
    fn render(&mut self, window: &mut RenderWindow, time: i32) {
        self.last_frame_at = self.last_frame_at + time;
        let index = (self.last_frame_at + time) / 100;
        let (offset_x, offset_y) = self.offset;
        let animation_frame_offset = (((index % self.frames) as f32 * 19.5f32) as i32) + 2;
        let frame = IntRect::new(animation_frame_offset + offset_x, offset_y, self.width, self.height);
        let (x,y) = self.position;
        self.sprite.set_position2f(x, y);
        self.sprite.set_texture_rect(&frame);

        window.draw(&(self.sprite));
    }
}



pub struct FrameStats {
    last_measure_at: i32,
    frames: i32,
    text: Text,
}

impl FrameStats {
    pub fn new() -> FrameStats {
        let font = Font::new_from_file("resources/sansation.ttf").unwrap();
        let text = Text::new_init("frames/s: 0.0", Rc::new(RefCell::new(font)), 12).unwrap();

        FrameStats {
            frames: 0,
            last_measure_at: 0,
            text: text
        }
    }
}

impl Renderable for FrameStats {
    fn render(&mut self, window: &mut RenderWindow, elapsed: i32) {
        self.last_measure_at += elapsed;
        self.frames += 1;

        if self.last_measure_at > 1000 {
            let rate = (self.frames * 1000) as f32 / self.last_measure_at as f32;
            self.text.set_string(&format!("frames/s: {}", rate));
            self.last_measure_at = 0;
            self.frames = 0;
        }

        window.draw(&self.text);
    }
}

pub enum UnitAction {
    Idle, Moving
}

pub struct Unit {
    up:    AnimatedSprite,
    down:  AnimatedSprite,
    left:  AnimatedSprite,
    right: AnimatedSprite,
    current_action: UnitAction,
    direction: Key
}

impl Renderable for Unit {
    fn render(&mut self, window: &mut RenderWindow, elapsed: i32) {
        match self.current_action {
            UnitAction::Idle => {
                self.render_moving(window, 0);
            },
            UnitAction::Moving => {
                self.render_moving(window, elapsed);
            },
        };
    }
}

impl Unit {
    pub fn new(
        up:    AnimatedSprite,
        down:  AnimatedSprite,
        left:  AnimatedSprite,
        right: AnimatedSprite
    ) -> Unit {
        Unit {
            up:    up,
            down:  down,
            left:  left,
            right: right,
            direction: Key::Down,
            current_action: UnitAction::Idle
        }
    }

    pub fn consume_input(&mut self, key: Key) {
        match key {
            Key::Up | Key::Down | Key::Left | Key::Right => {
                self.direction = key;
                self.current_action = UnitAction::Moving;
            }
            _ => {}
        };
    }

    fn render_moving(&mut self, window: &mut RenderWindow, elapsed: i32) {
        match self.direction {
            Key::Up    => self.up.render(window, elapsed),
            Key::Down  => self.down.render(window, elapsed),
            Key::Right => self.right.render(window, elapsed),
            Key::Left  => self.left.render(window, elapsed),
            _ => {}
        }
    }
}
