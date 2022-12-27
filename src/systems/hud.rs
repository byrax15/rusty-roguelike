use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query
        = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .next()
        .unwrap();
    DrawBatch::new()
        .target(2)
        .print_centered(1, "Explore the Dungeon. Cursor keys to move.")
        .bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            player_health.current,
            player_health.max,
            ColorPair::new(RED, BLACK))
        .print_color_centered(
            0,
            format!(" Health: {} / {} ", player_health.current, player_health.max),
            ColorPair::new(WHITE, RED))
        .submit(10_000)
        .expect("Batch error");
}