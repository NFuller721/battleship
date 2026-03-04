use crate::board::AttackResponse;

pub struct Ship {
    pub(crate) latitude: u32,
    pub(crate) longitude: u32,
    pub(crate) direction: Direction,
    pub(crate) hits: u8,
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
            hits: 0,
        }
    }

    pub fn hit(&mut self, latitude: u32, longitude: u32, length: u8) -> AttackResponse {
        match self.direction {
            Direction::Horizontal => {
                if self.longitude != longitude {
                    return AttackResponse::Miss;
                };

                let end = self.latitude + length as u32;
                if !(self.latitude..=end).contains(&latitude) {
                    return AttackResponse::Miss;
                }

                self.hits += 1;
                if self.hits >= length { 
                    return AttackResponse::Sink;
                }

                AttackResponse::Hit
            }
            Direction::Vertical => {
                // For this function I had to swap lat/lon because I forgot
                if self.latitude != latitude {
                    return AttackResponse::Miss;
                };

                let end = self.longitude + length as u32;
                // The value that was checked against contains was wrongly latitude
                if !(self.longitude..=end).contains(&longitude) {
                    return AttackResponse::Miss;
                }

                self.hits += 1;
                if self.hits >= length { 
                    return AttackResponse::Sink;
                }

                AttackResponse::Hit
            }
        }
    }
}
