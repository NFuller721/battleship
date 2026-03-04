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
