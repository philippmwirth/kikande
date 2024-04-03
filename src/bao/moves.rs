//! Bao moves.

use crate::bao::direction::Direction;
use std::fmt::{Display, Formatter, Result};

pub mod flags {
    pub const DIRECTION_R: u8 = 0b0000_0001;
    pub const NAMUA: u8 = 0b0000_0010;
    pub const CAPTURE: u8 = 0b0000_0100;
    pub const RELAY: u8 = 0b0000_1000;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Move {
    pub index: i8,
    pub flags: u8,
}

impl Move {
    pub fn new(index: i8, flags: u8) -> Move {
        Move { index, flags }
    }

    pub fn mtaji_capture_right(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::CAPTURE | flags::DIRECTION_R,
        }
    }

    pub fn mtaji_capture_left(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::CAPTURE,
        }
    }

    pub fn mtaji_relay_right(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::DIRECTION_R,
        }
    }

    pub fn mtaji_relay_left(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: 0,
        }
    }

    pub fn namua_capture_right(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::CAPTURE | flags::DIRECTION_R | flags::NAMUA,
        }
    }

    pub fn namua_capture_left(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::CAPTURE | flags::NAMUA,
        }
    }

    pub fn namua_relay_right(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::DIRECTION_R | flags::NAMUA,
        }
    }

    pub fn namua_relay_left(source_index: i8) -> Move {
        Move {
            index: source_index,
            flags: flags::NAMUA,
        }
    }

    pub fn get_direction(&self) -> Direction {
        if self.flags & flags::DIRECTION_R != 0 {
            self.get_direction_r()
        } else {
            self.get_direction_l()
        }
    }

    fn get_direction_r(self) -> Direction {
        if self.flags & flags::CAPTURE != 0 {
            Direction::CounterClockwise
        } else {
            Direction::Clockwise
        }
    }

    fn get_direction_l(self) -> Direction {
        if self.flags & flags::CAPTURE != 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        }
    }

    pub fn is_namua(&self) -> bool {
        self.flags & flags::NAMUA != 0
    }

    pub fn is_capture(&self) -> bool {
        self.flags & flags::CAPTURE != 0
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Get the row of the move if necessary.
        let row = match (self.index, self.flags & flags::NAMUA) {
            (0..=7, 0) => "A",
            (8..=15, 0) => "B",
            _ => "",
        };
        // Get the direction of the move.
        let direction = match (
            self.index,
            self.flags & flags::DIRECTION_R,
            self.flags & flags::CAPTURE,
        ) {
            (_, direction, 0) => {
                // No capture.
                if direction == 0 {
                    "L"
                } else {
                    "R"
                }
            }
            (0..=1, _, _) => "",
            (6..=7, _, _) => "",
            (_, direction, _) => {
                // Capture.
                if direction == 0 {
                    "L"
                } else {
                    "R"
                }
            }
        };
        // Get the index of the move.
        let index = match self.index {
            0..=7 => self.index + 1,
            8..=15 => self.index - 7,
            _ => panic!("Invalid index: {}", self.index),
        };

        write!(f, "{}{}{}", row, index, direction)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_direction() {
        // No capture, front row, moving right
        let right_move = Move {
            index: 0,
            flags: flags::DIRECTION_R,
        };
        assert_eq!(right_move.get_direction(), Direction::Clockwise);
        // No capture, front row, moving left
        let right_move = Move { index: 0, flags: 0 };
        assert_eq!(right_move.get_direction(), Direction::CounterClockwise);
        // Capture, front row, moving right
        let right_move = Move {
            index: 0,
            flags: flags::DIRECTION_R | flags::CAPTURE,
        };
        assert_eq!(right_move.get_direction(), Direction::CounterClockwise);
        // Capture, front row, moving left
        let right_move = Move {
            index: 0,
            flags: flags::CAPTURE,
        };
        assert_eq!(right_move.get_direction(), Direction::Clockwise);
    }

    #[test]
    fn test_is_namua() {
        let move1 = Move {
            index: 0,
            flags: flags::NAMUA,
        };
        assert!(move1.is_namua());
        let move2 = Move { index: 0, flags: 0 };
        assert!(!move2.is_namua());
    }

    #[test]
    fn test_is_capture() {
        let move1 = Move {
            index: 0,
            flags: flags::CAPTURE,
        };
        assert!(move1.is_capture());
        let move2 = Move { index: 0, flags: 0 };
        assert!(!move2.is_capture());
    }
}
