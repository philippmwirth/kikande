//! Bao board.

use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref MASHUMO_RANDS: [u64; 16] = {
        let mut rng = rand::thread_rng();
        let mut mashumo = [0u64; 16];
        for shumo in &mut mashumo {
            *shumo = rng.gen();
        }
        mashumo
    };
}

#[derive(Clone, Debug)]
pub struct Mashumo {
    pub mashumo: [u8; 16], // Only for one player.
    pub bitboard: u8,
    pub zobrist: u64,
}

impl Default for Mashumo {
    fn default() -> Self {
        Self::new()
    }
}

impl Mashumo {
    pub fn new() -> Mashumo {
        let zobrist = MASHUMO_RANDS[4].rotate_left(6)
            ^ MASHUMO_RANDS[5].rotate_left(2)
            ^ MASHUMO_RANDS[6].rotate_left(2);
        Mashumo {
            mashumo: [0, 0, 0, 0, 6, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            bitboard: 0b00001110,
            zobrist,
        }
    }

    pub fn get_and_empty_seeds(&mut self, index: i8) -> u8 {
        let value = self.mashumo[index as usize];
        self.zobrist ^= MASHUMO_RANDS[index as usize].rotate_left(value as u32);
        self.mashumo[index as usize] = 0;
        self.zobrist ^= MASHUMO_RANDS[index as usize];
        if index < 8 {
            self.bitboard &= !(1 << (7 - index));
        }
        value
    }

    pub fn get_seeds(&self, index: i8) -> u8 {
        self.mashumo[index as usize]
    }

    pub fn increment_seeds(&mut self, index: i8) {
        self.zobrist ^=
            MASHUMO_RANDS[index as usize].rotate_left(self.mashumo[index as usize] as u32);
        self.mashumo[index as usize] = self.mashumo[index as usize].wrapping_add(1);
        self.zobrist ^=
            MASHUMO_RANDS[index as usize].rotate_left(self.mashumo[index as usize] as u32);
        if index < 8 {
            self.bitboard |= 1 << (7 - index);
        }
    }

    pub fn set_seeds(&mut self, index: i8, value: u8) {
        self.zobrist ^=
            MASHUMO_RANDS[index as usize].rotate_left(self.mashumo[index as usize] as u32);
        self.mashumo[index as usize] = value;
        self.zobrist ^=
            MASHUMO_RANDS[index as usize].rotate_left(self.mashumo[index as usize] as u32);
        if index < 8 && self.mashumo[index as usize] > 0 {
            self.bitboard |= 1 << (7 - index);
        } else if index < 8 {
            self.bitboard &= !(1 << (7 - index));
        }
    }

    pub fn get_shumo_cw(&self, count: usize, index: i8) -> (u8, i8) {
        self.iter_cw(count, index, 1)
            .last()
            .expect("Empty shumo iterator")
    }

    pub fn get_shumo_ccw(&self, count: usize, index: i8) -> (u8, i8) {
        self.iter_ccw(count, index, 1)
            .last()
            .expect("Empty shumo iterator")
    }

    pub fn get_shumo_opposite(&self, index: i8) -> Option<i8> {
        match index {
            0..=7 => Some(7 - index),
            _ => None,
        }
    }

    pub fn iter_index_cw(&self, count: usize, index: i8) -> impl Iterator<Item = usize> {
        (0..count).map(move |i| (index + i as i8) as usize % 16)
    }

    pub fn iter_index_ccw(&self, count: usize, index: i8) -> impl Iterator<Item = usize> {
        (0..count).map(move |i| ((index - i as i8) + 16) as usize % 16)
    }

    pub fn iter_cw(
        &self,
        count: usize,
        index: i8,
        _stride: i8,
    ) -> impl Iterator<Item = (u8, i8)> + '_ {
        self.iter_index_cw(count, index)
            .map(move |i| (self.mashumo[i], i as i8))
    }

    pub fn iter_ccw(
        &self,
        count: usize,
        index: i8,
        _stride: i8,
    ) -> impl Iterator<Item = (u8, i8)> + '_ {
        self.iter_index_ccw(count, index)
            .map(move |i| (self.mashumo[i], i as i8))
    }

    pub fn sow_cw(&mut self, index: i8, seeds: usize) -> i8 {
        let mut end_index = index;
        for index in self.iter_index_cw(seeds, index) {
            self.increment_seeds(index as i8);
            end_index = index as i8;
        }
        end_index
    }

    pub fn sow_ccw(&mut self, index: i8, seeds: usize) -> i8 {
        let mut end_index = index;
        for index in self.iter_index_ccw(seeds, index) {
            self.increment_seeds(index as i8);
            end_index = index as i8;
        }
        end_index
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_and_empty_seeds() {
        let mut mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let value = mashumo.get_and_empty_seeds(3);
        assert_eq!(value, 3);
        assert_eq!(mashumo.mashumo[3], 0);
    }

    #[test]
    fn test_get_seeds() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let value = mashumo.get_seeds(3);
        assert_eq!(value, 3);
    }

    #[test]
    fn test_increment_seeds() {
        let mut mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        mashumo.increment_seeds(3);
        assert_eq!(mashumo.mashumo[3], 4);
    }

    #[test]
    fn test_set_seeds() {
        let mut mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        mashumo.set_seeds(3, 10);
        assert_eq!(mashumo.mashumo[3], 10);
    }

    #[test]
    fn test_mashumo_get_shumo_cw() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let (value, index) = mashumo.get_shumo_cw(3, 0);
        assert_eq!(value, 2);
        assert_eq!(index, 2);
        let (value, index) = mashumo.get_shumo_cw(6, 14);
        assert_eq!(value, 3);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_mashumo_get_shumo_ccw() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let (value, index) = mashumo.get_shumo_ccw(3, 15);
        assert_eq!(value, 13);
        assert_eq!(index, 13);
        let (value, index) = mashumo.get_shumo_ccw(6, 1);
        assert_eq!(value, 12);
        assert_eq!(index, 12);
    }

    #[test]
    fn test_get_shumo_opposite() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let index = mashumo.get_shumo_opposite(0);
        assert_eq!(index, Some(7));
        let index = mashumo.get_shumo_opposite(7);
        assert_eq!(index, Some(0));
        let index = mashumo.get_shumo_opposite(8);
        assert_eq!(index, None);
    }

    #[test]
    fn test_mashumo_iter_index_cw() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let indices: Vec<usize> = mashumo.iter_index_cw(3, 0).collect();
        assert_eq!(indices, vec![0, 1, 2]);
        let indices: Vec<usize> = mashumo.iter_index_cw(6, 14).collect();
        assert_eq!(indices, vec![14, 15, 0, 1, 2, 3]);
    }

    #[test]
    fn test_mashumo_iter_index_ccw() {
        let mashumo = Mashumo {
            mashumo: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            bitboard: 0b01111111,
            zobrist: 0,
        };
        let indices: Vec<usize> = mashumo.iter_index_ccw(3, 15).collect();
        assert_eq!(indices, vec![15, 14, 13]);
        let indices: Vec<usize> = mashumo.iter_index_ccw(6, 1).collect();
        assert_eq!(indices, vec![1, 0, 15, 14, 13, 12]);
    }

    #[test]
    fn test_sow_cw() {
        let mut mashumo = Mashumo {
            mashumo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            bitboard: 0b00000000,
            zobrist: 0,
        };
        let index = mashumo.sow_cw(0, 3);
        assert_eq!(index, 2);
        assert_eq!(
            mashumo.mashumo,
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        let index = mashumo.sow_cw(14, 6);
        assert_eq!(index, 3);
        assert_eq!(
            mashumo.mashumo,
            [2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]
        );
    }

    #[test]
    fn test_sow_ccw() {
        let mut mashumo = Mashumo {
            mashumo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            bitboard: 0b00000000,
            zobrist: 0,
        };
        let index = mashumo.sow_ccw(15, 3);
        assert_eq!(index, 13);
        assert_eq!(
            mashumo.mashumo,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1]
        );
        let index = mashumo.sow_ccw(1, 6);
        assert_eq!(index, 12);
        assert_eq!(
            mashumo.mashumo,
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2]
        );
    }
}
