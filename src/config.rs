//! Configuration for the search algorithm.

/// Configuration for the search algorithm.
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// The maximum depth to search in half-moves.
    pub max_depth: u8,
    /// The number of threads to use for parallel iterative deepening.
    pub num_threads: u8,
    /// The maximum time to search in milliseconds.
    pub max_time_ms: Option<u32>,
}

impl Default for SearchConfig {
    /// Returns the default search configuration.
    fn default() -> Self {
        Self::new(20, 1, None)
    }
}

impl SearchConfig {
    /// Creates a new search configuration.
    pub fn new(max_depth: u8, num_threads: u8, max_time_ms: Option<u32>) -> Self {
        SearchConfig {
            max_depth,
            num_threads,
            max_time_ms,
        }
    }

    /// Creates a new search configuration based on a difficulty level.
    pub fn new_from_difficulty(difficulty: u8) -> Self {
        let max_depth = match difficulty {
            0..=1 => 1,
            2 => 2,
            3 => 10,
            4 => 10,
            5 => 10,
            6 => 10,
            7 => 20,
            8 => 20,
            9 => 20,
            10 => 40,
            _ => 40,
        };
        let max_time_ms = match difficulty {
            1 => Some(5),
            2 => Some(10),
            3 => Some(20),
            4 => Some(50),
            5 => Some(100),
            6 => Some(200),
            7 => Some(500),
            8 => Some(1000),
            9 => Some(2000),
            10 => Some(3000),
            _ => None,
        };
        SearchConfig::new(max_depth, 1, max_time_ms)
    }
}
