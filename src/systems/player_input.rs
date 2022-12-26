use crate::prelude::*;
use VirtualKeyCode::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState
) {
    if let Some(key) = key {
        let delta = match key {
            Left => Point::new(-1, 0),
            Right => Point::new(1, 0),
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players
                = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let dest = *pos+delta;
                if map.can_enter_tile(dest) {
                    *pos = dest;
                    camera.on_player_move(dest);
                    *turn_state = TurnState::PlayerTurn;
                }
            });
        }
    }
}