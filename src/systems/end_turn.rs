use crate::prelude::*;
use TurnState::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    *turn_state = match turn_state {
        AwaitingInput => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
    };
}