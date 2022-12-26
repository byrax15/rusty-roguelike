use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = camera.top_left();
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}