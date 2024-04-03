use crate::bao::moves::Move;
use crate::bao::pv::PVLine;
use crate::search::node::Node;
use rustc_hash::FxHashMap;
use std::sync::RwLock;

#[derive(Clone, Copy, PartialEq)]
pub enum EntryType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone)]
pub struct TranspositionTableEntry {
    pub depth: u8,
    pub score: f32,
    pub best_move: Move,
    pub entry_type: EntryType,
}

pub struct TranspositionTable {
    maps: Vec<RwLock<FxHashMap<u64, TranspositionTableEntry>>>,
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}

static TABLE_SIZE: usize = 128;

fn mul_hi64(a: u64, b: u64) -> u64 {
    #[cfg(target_pointer_width = "64")]
    {
        // Use Rust's 128-bit integers directly when available
        (((a as u128) * (b as u128)) >> 64_u64)
            .try_into()
            .expect("u64 overflow")
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        // Fallback manual computation for platforms without direct 128-bit support
        let aL = a as u32;
        let aH = (a >> 32) as u32;
        let bL = b as u32;
        let bH = (b >> 32) as u32;
        let c1 = ((aL as u64) * (bL as u64)) >> 32;
        let c2 = (aH as u64) * (bL as u64) + c1;
        let c3 = (aL as u64) * (bH as u64) + (c2 as u32) as u64;
        (aH as u64) * (bH as u64) + (c2 >> 32) + (c3 >> 32)
    }
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            maps: (0..TABLE_SIZE)
                .map(|_| RwLock::new(FxHashMap::default()))
                .collect(),
        }
    }

    pub fn insert(&self, key: u64, best_move: &Move, depth: u8, score: f32, entry_type: EntryType) {
        let table_index = mul_hi64(key, TABLE_SIZE as u64);
        let mut table = self.maps[table_index as usize]
            .write()
            .expect("Failed to lock table");
        table
            .entry(key)
            .and_modify(|e| {
                if e.depth < depth {
                    *e = TranspositionTableEntry {
                        depth,
                        best_move: *best_move,
                        score,
                        entry_type,
                    };
                }
            })
            .or_insert_with(|| TranspositionTableEntry {
                depth,
                best_move: *best_move,
                score,
                entry_type,
            });
    }

    pub fn probe(&self, key: u64) -> Option<TranspositionTableEntry> {
        let table_index = mul_hi64(key, TABLE_SIZE as u64);
        let table = self.maps[table_index as usize]
            .read()
            .expect("Failed to lock table");
        table.get(&key).cloned()
    }

    pub fn get_pv_line(&self, node: &Node, depth: u8) -> PVLine {
        let mut pv_line = PVLine {
            moves: vec![],
            value: 0.0,
        };
        let mut current_node = node.clone();
        for _ in 0..depth {
            let table_index = mul_hi64(current_node.zobrist, TABLE_SIZE as u64);
            let table = self.maps[table_index as usize]
                .read()
                .expect("Failed to lock table");
            let entry = match table.get(&current_node.zobrist) {
                Some(entry) => entry,
                None => break,
            };
            pv_line.moves.push(entry.best_move);
            pv_line.value = entry.score;
            current_node = current_node.apply_move(&entry.best_move);
        }
        pv_line
    }
}
