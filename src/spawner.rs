use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player, pos, FieldOfView::new(8),
        Health { current: 10, max: 10 },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy, pos, ChasingPlayer, Name(name),
        FieldOfView::new(6),
        Health { current: hp, max: hp },
        Render { glyph, color: ColorPair::new(WHITE, BLACK) }
    ));
}

pub fn spawn_amulet_of_yala(
    ecs: &mut World,
    pos: Point,
) {
    ecs.push((
        Item, AmuletOfYala, pos,
        Name("Amulet of Yala".to_string()),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
    ));
}

pub fn spawn_entity(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point,
) {
    match rng.roll_dice(1, 6) {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    };
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item, pos,
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item, pos, ProvidesDungeonMap,
        Name("Dungeon Map".to_string()),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
    ));
}


fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}