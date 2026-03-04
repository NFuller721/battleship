# Interface

```rust
use battleship::{Game, Board, Ship, Direction};

fn main() {
    let p1 = Board::new()
        .add_carrier(Ship::new(2, 9, Direction::Horizontal))
        .add_battleship(Ship::new(4, 2, Direction::Vertical))
        .add_cruiser(Ship::new(1, 3, Direction::Vertical))
        .add_submarine(Ship::new(3, 7, Direction::Horizontal))
        .add_destroyer(Ship::new(4, 1, Direction::Horizontal))
        .build();
    let p2 = Board::new()
        .add_carrier(Ship::new(2, 9, Direction::Horizontal))
        .add_battleship(Ship::new(4, 2, Direction::Vertical))
        .add_cruiser(Ship::new(1, 3, Direction::Vertical))
        .add_submarine(Ship::new(3, 7, Direction::Horizontal))
        .add_destroyer(Ship::new(4, 1, Direction::Horizontal))
        .build();
    let game = Game::new(p1, p2).start();

    // Player One starts attack with (2, 9).
    game.attack(2, 9);

    // Player Two attacks with (3, 9).
    game.attack(3, 9);

    // Player One attacks with (4, 9).
    game.attack(4, 9);

    // Player Two attacks with (5, 9).
    game.attack(5, 9);

    // Player One attacks with (6, 9).
    game.attack(6, 9);
}
```

# Order

1. Designing the `Board` X
2. Creating a `Game` structure X
3. Making the `Ship` and `Direction`
4. Validate ship placement and builder
5. Designing the attack system
6. Validate the attacks
