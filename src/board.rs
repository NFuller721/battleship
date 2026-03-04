use std::marker::PhantomData;

use crate::ship::Ship;

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

impl<Carrier, Battleship, Cruiser, Submarine, Destroyer> BoardBuilder<Carrier, Battleship, Cruiser, Submarine, Destroyer> {
    pub fn add_carrier(self, ship: Ship) -> BoardBuilder<CarrierSet, Battleship, Cruiser, Submarine, Destroyer> {
        BoardBuilder {
            carrier: Some(ship),
            battleship: None,
            cruiser: None,
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_battleship(self, ship: Ship) -> BoardBuilder<Carrier, BattleshipSet, Cruiser, Submarine, Destroyer> {
        BoardBuilder {
            carrier: None,
            battleship: Some(ship),
            cruiser: None,
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_cruiser(self, ship: Ship) -> BoardBuilder<Carrier, Battleship, CruiserSet, Submarine, Destroyer> {
        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: Some(ship),
            submarine: None,
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_submarine(self, ship: Ship) -> BoardBuilder<Carrier, Battleship, Cruiser, SubmarineSet, Destroyer> {
        BoardBuilder {
            carrier: None,
            battleship: None,
            cruiser: None,
            submarine: Some(ship),
            destroyer: None,
            _marker: PhantomData,
        }
    }

    pub fn add_destroyer(self, ship: Ship) -> BoardBuilder<Carrier, Battleship, Cruiser, Submarine, DestroyerSet> {
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
