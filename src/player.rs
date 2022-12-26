use crate::prelude::*;
use VirtualKeyCode as VKC;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Player {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'));
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if ctx.key.is_none() {
            return;
        }
        let delta = match ctx.key.unwrap() {
            VKC::Left => Point::new(-1, 0),
            VKC::Right => Point::new(1, 0),
            VKC::Up => Point::new(0, -1),
            VKC::Down => Point::new(0, 1),
            _ => Point::zero(),
        };
        let new_pos = self.position + delta;
        if map.can_enter_tile(new_pos) {
            self.position = new_pos;
            camera.on_player_move(new_pos);
        }
    }
}
