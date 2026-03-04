pub struct Ship {
    latitude: u32,
    longitude: u32,
    direction: Direction,
}

pub enum Direction {
    Vertical,
    Horizontal,
}
