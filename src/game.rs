use std::marker::PhantomData;

use crate::board::{AttackResponse, Board};

pub struct Game<T> {
    attacker: Board,
    defender: Board,
    player_turn: PhantomData<T>,
}

pub struct PlayerOne;
pub struct PlayerTwo;

impl<T> Game<T> {
    pub fn new(attacker: Board, defender: Board) -> Game<PlayerOne> {
        Game {
            attacker,
            defender,
            player_turn: PhantomData::<PlayerOne>,
        }
    }
}

impl Game<PlayerOne> {
    pub fn attack(mut self, lat: u32, lon: u32) -> Game<PlayerTwo> {
        let response = self.defender.hit(lat, lon);
        match response {
            AttackResponse::Hit => println!("player one hit player two's ship at ({lat}, {lon})"),
            AttackResponse::Sink => println!("player one sunk player two's ship at ({lat}, {lon})"),
            AttackResponse::Miss => println!("player one missed at ({lat}, {lon})"),
        }

        Game {
            attacker: self.attacker,
            defender: self.defender,
            player_turn: PhantomData,
        }
    }
}

impl Game<PlayerTwo> {
    pub fn attack(mut self, lat: u32, lon: u32) -> Game<PlayerOne> {
        let response = self.attacker.hit(lat, lon);

        // Had to change the text from "player one ..." to "player two ..."
        match response {
            AttackResponse::Hit => println!("player two hit player one's ship at ({lat}, {lon})"),
            AttackResponse::Sink => println!("player two sunk player one's ship at ({lat}, {lon})"),
            AttackResponse::Miss => println!("player two missed at ({lat}, {lon})"),
        }

        Game {
            attacker: self.attacker,
            defender: self.defender,
            player_turn: PhantomData,
        }
    }
}
