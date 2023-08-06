use crate::prelude::*;
pub fn end_turn(
    turn_state: ResMut<State<TurnState>>,
    mut next_state:ResMut<NextState<TurnState>>,
){
    let current_state: TurnState = turn_state.clone();
    let mut new_state = match *turn_state.get() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        //TurnState::StartScreen => return,
        //TurnState::NextLevel => TurnState::AwaitingInput,
        _ => current_state
    };
    next_state.set(new_state);
}