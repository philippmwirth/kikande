//! Principal variation lines.

use crate::bao::moves::Move;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub struct PVLine {
    pub moves: Vec<Move>,
    pub value: f32,
}

impl PVLine {
    pub fn update(&mut self, m: Option<Move>, pvline: PVLine, value: f32) {
        self.moves = vec![];
        if let Some(m) = m {
            self.moves.push(m);
        }
        self.moves.extend(pvline.moves);
        self.value = value;
    }

    pub fn get_depth(&self) -> usize {
        self.moves.len()
    }
}

impl PartialOrd for PVLine {
    fn partial_cmp(&self, other: &PVLine) -> Option<Ordering> {
        if self.get_depth() == other.get_depth() {
            self.value.partial_cmp(&other.value)
        } else {
            Some(other.moves.len().cmp(&self.moves.len()))
        }
    }
}

impl Display for PVLine {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sign = match self.value >= 0.0 {
            true => "+/=",
            false => "-/=",
        };
        let mut line = format!("{}({:.2}): ", sign, self.value);
        for (i, m) in self.moves.clone().into_iter().enumerate() {
            let suffix = if i % 2 > 0 {
                ";".to_string()
            } else {
                "".to_string()
            };
            line.push_str(&format!("{}{} ", m, suffix));
        }
        write!(f, "{}", line)
    }
}
