use crate::prelude::*;
use TurnState::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
) {
    let mut player_hp
        = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet
        = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet
        .iter(ecs)
        .nth(0)
        .unwrap_or(&amulet_default);

    let mut player_is_dead = false;
    let mut amulet_found = false;
    let mut exit_found = false;

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        player_is_dead = hp.current < 1;
        amulet_found = amulet_found || pos == amulet_pos;
        exit_found = exit_found || map.tiles[map.point2d_to_index(*pos)] == TileType::Exit;
    });

    *turn_state = match (*turn_state, player_is_dead, amulet_found, exit_found) {
        (.., true, _) => Victory,
        (_, true, ..) => GameOver,
        (.., true) => NextLevel,
        (AwaitingInput, ..) => return,
        (PlayerTurn, ..) => MonsterTurn,
        (MonsterTurn, ..) => AwaitingInput,
        (GameOver, ..) => panic!("Game not reset properly"),
        (Victory, ..) => panic!("Game not reset properly"),
        (NextLevel, ..) => panic!("Game not reset properly"),
    };
}

