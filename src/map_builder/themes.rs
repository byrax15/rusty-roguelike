use crate::prelude::*;
use crate::TileType::*;

pub struct DungeonTheme;

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            Floor => to_cp437('.'),
            Wall => to_cp437('#'),
        }
    }
}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self)
    }
}

pub struct ForestTheme;

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            Floor => to_cp437(';'),
            Wall => to_cp437('"'),
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self)
    }
}