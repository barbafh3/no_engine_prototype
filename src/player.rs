use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
};

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub fn update_player(keyboard_event: &Event, player: &mut Player) {
    let mut direction: Point = Point::new(0, 0);
    match keyboard_event {
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => direction += Point::new(-1, 0),
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => direction += Point::new(1, 0),
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => direction += Point::new(0, -1),
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => direction += Point::new(0, 1),
        _ => {}
    }
    player.position = player.position.offset(direction.x(), direction.y());
}
