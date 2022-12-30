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
) {
    let mut player_hp
        = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet
        = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    let mut player_is_dead = false;
    let mut amulet_found = false;

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        player_is_dead = hp.current < 1;
        amulet_found = amulet_found || pos == amulet_pos;
    });

    *turn_state = match (*turn_state, player_is_dead, amulet_found) {
        (_, _, true) => Victory,
        (_, true, _) => GameOver,
        (AwaitingInput, _, _) => return,
        (PlayerTurn, _, _) => MonsterTurn,
        (MonsterTurn, _, _) => AwaitingInput,
        (GameOver, _, _) => panic!("Game not reset properly"),
        (Victory, _, _,) => panic!("Game not reset properly"),
    };
}

