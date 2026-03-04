use std::marker::PhantomData;

use crate::board::Board;

pub struct Game<T> {
    attacker: Board,
    defender: Board,
    player_turn: PhantomData<T>,
}

pub(crate) struct PlayerOne;
pub(crate) struct PlayerTwo;
