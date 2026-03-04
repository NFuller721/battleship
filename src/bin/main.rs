use battleship::ship::{Ship, Direction};
use battleship::game::{Game, PlayerOne};
use battleship::board::Board;

fn main() {
    let p1 = Board::builder()
        .add_carrier(Ship::new(2, 9, Direction::Vertical))
        .add_battleship(Ship::new(4, 2, Direction::Vertical))
        .add_cruiser(Ship::new(1, 3, Direction::Vertical))
        .add_submarine(Ship::new(3, 7, Direction::Horizontal))
        .add_destroyer(Ship::new(4, 1, Direction::Horizontal))
        .build();
    let p2 = Board::builder()
        .add_carrier(Ship::new(2, 9, Direction::Horizontal))
        .add_battleship(Ship::new(4, 2, Direction::Vertical))
        .add_cruiser(Ship::new(1, 3, Direction::Vertical))
        .add_submarine(Ship::new(3, 7, Direction::Horizontal))
        .add_destroyer(Ship::new(4, 1, Direction::Horizontal))
        .build();
    let game = Game::<PlayerOne>::new(p1, p2);

    let game = game.attack(2, 9);
    let game = game.attack(2, 9);
    let game = game.attack(3, 9);
    let game = game.attack(3, 9);
    let game = game.attack(4, 9);
    let game = game.attack(4, 9);
    let game = game.attack(5, 9);
    let game = game.attack(5, 9);
    let game = game.attack(6, 9);
    let _ = game.attack(6, 9);
}
