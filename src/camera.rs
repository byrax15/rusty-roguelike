use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let mut s = Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
        };

        refresh_camera(&mut s, player_position);
        s
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        refresh_camera(self, player_position)
    }

    pub fn top_left(&self) -> Point {
        Point::new(self.left_x, self.top_y)
    }
}

fn refresh_camera(c: &mut Camera, player_position: Point) {
    c.left_x = player_position.x - DISPLAY_WIDTH / 2;
    c.right_x = player_position.x + DISPLAY_WIDTH / 2;
    c.top_y = player_position.y - DISPLAY_HEIGHT / 2;
    c.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
}