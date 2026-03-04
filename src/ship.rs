pub struct Ship {
    pub(crate) latitude: u32,
    pub(crate) longitude: u32,
    pub(crate) direction: Direction,
}

pub enum Direction {
    Vertical,
    Horizontal,
}

impl Ship {
    pub fn new(latitude: u32, longitude: u32, direction: Direction) -> Ship {
        assert!(latitude >= 1);
        assert!(longitude >= 1);

        Ship {
            latitude,
            longitude,
            direction,
        }
    }
}
