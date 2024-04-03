use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Direction::Clockwise => write!(f, "CW"),
            Direction::CounterClockwise => write!(f, "CCW"),
        }
    }
}
