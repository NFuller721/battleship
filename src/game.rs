use std::marker::PhantomData;

use crate::board::Board;

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
    pub fn attack(self, lat: u32, lon: u32) -> Game<PlayerTwo> {
        let response = self.defender.hit(lat, lon);
        match response {
            AttackResponse::Hit(title) => println!("player one hit player two's {title} at ({lat}, {lon})");
            AttackResponse::Sink(title) => println!("player one sunk player two's {title} at ({lat}, {lon})");
            AttackResponse::Miss => println!("player one missed at ({lat, lon})");
        }
    }
}

impl Game<PlayerTwo> {
    pub fn attack(self, lat: u32, lon: u32) -> Game<PlayerOne> {
        let response = self.attacker.hit(lat, lon);
        match response {
            AttackResponse::Hit(title) => println!("player one hit player two's {title} at ({lat}, {lon})");
            AttackResponse::Sink(title) => println!("player one sunk player two's {title} at ({lat}, {lon})");
            AttackResponse::Miss => println!("player one missed at ({lat, lon})");
        }
    }
}
