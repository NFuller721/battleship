use std::marker::PhantomData;

use crate::ship::{Direction, Ship};

pub struct Board {
    carrier: Ship,
    battleship: Ship,
    cruiser: Ship,
    submarine: Ship,
    destroyer: Ship,
    attacks: Vec<Attack>,
}

pub(crate) struct Attack {
    latitude: u32,
    longitude: u32,
}

impl Board {
    pub fn builder() -> BoardBuilder<Unset, Unset, Unset, Unset, Unset> {
        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: None,
            submarine: None,
            destroyer: None,
            _marker: PhantomData::<(Unset, Unset, Unset, Unset, Unset)>,
        }
    }
}

pub struct CarrierSet;
pub struct BattleshipSet;
pub struct CruiserSet;
pub struct SubmarineSet;
pub struct DestroyerSet;
pub struct Unset;

pub struct BoardBuilder<Carrier, Battleship, Cruiser, Submarine, Destroyer> {
    carrier: Option<Ship>,
    battleship: Option<Ship>,
    cruiser: Option<Ship>,
    submarine: Option<Ship>,
    destroyer: Option<Ship>,
    _marker: PhantomData<(Carrier, Battleship, Cruiser, Submarine, Destroyer)>,
}

impl<Carrier, Battleship, Cruiser, Submarine, Destroyer>
    BoardBuilder<Carrier, Battleship, Cruiser, Submarine, Destroyer>
{
    pub fn add_carrier(
        self,
        ship: Ship,
    ) -> BoardBuilder<CarrierSet, Battleship, Cruiser, Submarine, Destroyer> {
        assert!(ship.latitude >= 1, "ship too far left");
        assert!(ship.longitude >= 1, "ship too high");

        match ship.direction {
            Direction::Vertical => {
                assert!(ship.latitude <= 15, "ship too far right");
                assert!(ship.longitude <= 11, "ship overflows the bottom");
            }
            Direction::Horizontal => {
                assert!(ship.latitude <= 11, "ship overflows the right");
                assert!(ship.longitude <= 15, "ship too far down");
            }
        };


        BoardBuilder {
            carrier: Some(ship),
            battleship: None,
            cruiser: None,
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_battleship(
        self,
        ship: Ship,
    ) -> BoardBuilder<Carrier, BattleshipSet, Cruiser, Submarine, Destroyer> {
        assert!(ship.latitude >= 1, "ship too far left");
        assert!(ship.longitude >= 1, "ship too high");

        match ship.direction {
            Direction::Vertical => {
                assert!(ship.latitude <= 15, "ship too far right");
                assert!(ship.longitude <= 12, "ship overflows the bottom");
            }
            Direction::Horizontal => {
                assert!(ship.latitude <= 12, "ship overflows the right");
                assert!(ship.longitude <= 15, "ship too far down");
            }
        };

        BoardBuilder {
            carrier: None,
            battleship: Some(ship),
            cruiser: None,
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_cruiser(
        self,
        ship: Ship,
    ) -> BoardBuilder<Carrier, Battleship, CruiserSet, Submarine, Destroyer> {
        assert!(ship.latitude >= 1, "ship too far left");
        assert!(ship.longitude >= 1, "ship too high");

        match ship.direction {
            Direction::Vertical => {
                assert!(ship.latitude <= 15, "ship too far right");
                assert!(ship.longitude <= 13, "ship overflows the bottom");
            }
            Direction::Horizontal => {
                assert!(ship.latitude <= 12, "ship overflows the right");
                assert!(ship.longitude <= 13, "ship too far down");
            }
        };

        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: Some(ship),
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_submarine(
        self,
        ship: Ship,
    ) -> BoardBuilder<Carrier, Battleship, Cruiser, SubmarineSet, Destroyer> {
        assert!(ship.latitude >= 1, "ship too far left");
        assert!(ship.longitude >= 1, "ship too high");

        match ship.direction {
            Direction::Vertical => {
                assert!(ship.latitude <= 15, "ship too far right");
                assert!(ship.longitude <= 13, "ship overflows the bottom");
            }
            Direction::Horizontal => {
                assert!(ship.latitude <= 12, "ship overflows the right");
                assert!(ship.longitude <= 13, "ship too far down");
            }
        };

        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: None,
            submarine: Some(ship),
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_destroyer(
        self,
        ship: Ship,
    ) -> BoardBuilder<Carrier, Battleship, Cruiser, Submarine, DestroyerSet> {
        assert!(ship.latitude >= 1, "ship too far left");
        assert!(ship.longitude >= 1, "ship too high");

        match ship.direction {
            Direction::Vertical => {
                assert!(ship.latitude <= 15, "ship too far right");
                assert!(ship.longitude <= 14, "ship overflows the bottom");
            }
            Direction::Horizontal => {
                assert!(ship.latitude <= 14, "ship overflows the right");
                assert!(ship.longitude <= 13, "ship too far down");
            }
        };

        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: None,
            submarine: None,
            destroyer: Some(ship),
            _marker: PhantomData,
        }
    }
}

impl BoardBuilder<CarrierSet, BattleshipSet, CruiserSet, SubmarineSet, DestroyerSet> {
    pub fn build(self) -> Board {
        Board {
            carrier: self.carrier.expect("carrier should be defined"),
            battleship: self.battleship.expect("battleship should be defined"),
            cruiser: self.cruiser.expect("cruiser should be defined"),
            submarine: self.submarine.expect("submarine should be defined"),
            destroyer: self.destroyer.expect("destroyer should be defined"),
            attacks: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
pub enum AttackResponse {
    Hit,
    Sink,
    Miss,
}

impl Board {
    pub fn hit(&mut self, latitude: u32, longitude: u32) -> AttackResponse {
        for attack in &self.attacks {
            if attack.latitude != latitude {
                continue;
            }

            if attack.longitude != longitude {
                continue;
            }

            panic!("can not make the same attack");
        }

        let response = self.hit_inner(latitude, longitude);

        self.attacks.push(Attack { latitude, longitude });

        response
    }

    fn hit_inner(&mut self, latitude: u32, longitude: u32) -> AttackResponse {
        let response = self.carrier.hit(latitude, longitude, 5);
        if response != AttackResponse::Miss {
            return response;
        }

        let response = self.battleship.hit(latitude, longitude, 5);
        if response != AttackResponse::Miss {
            return response;
        }

        let response = self.cruiser.hit(latitude, longitude, 5);
        if response != AttackResponse::Miss {
            return response;
        }

        let response = self.submarine.hit(latitude, longitude, 5);
        if response != AttackResponse::Miss {
            return response;
        }

        let response = self.destroyer.hit(latitude, longitude, 5);
        if response != AttackResponse::Miss {
            return response;
        }

        AttackResponse::Miss
    }
}
