use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            batch_tile(map, camera, &mut draw_batch, x, y);
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

fn batch_tile(map: &Map, camera: &Camera, draw_batch: &mut DrawBatch, x: i32, y: i32) {
    let pt = Point::new(x, y);
    let offset = Point::new(camera.left_x, camera.top_y);
    if map.in_bounds(pt) {
        let idx = map_idx(x, y);
        let glyph = match map.tiles[idx] {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        };
        draw_batch.set(
            pt - offset,
            ColorPair::new(WHITE, BLACK),
            glyph,
        );
    }
}