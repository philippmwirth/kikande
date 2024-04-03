use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    start_time: Instant,
    max_time_ms: u32,
    count: RefCell<u32>,
    is_time_up: RefCell<bool>,
}

impl Timer {
    pub fn new(max_time_ms: u32) -> Self {
        Timer {
            start_time: Instant::now(),
            max_time_ms,
            count: RefCell::new(0),
            is_time_up: RefCell::new(false),
        }
    }

    pub fn is_time_up(&self) -> bool {
        if *self.is_time_up.borrow() {
            return true;
        }

        // Increment count wrapped around 1024.
        self.count.replace_with(|&mut x| (x + 1) % 1024);
        let count = self.count.borrow();
        let is_time_up = match *count {
            0 => self.start_time.elapsed().as_millis() as u32 >= self.max_time_ms,
            _ => false,
        };
        *self.is_time_up.borrow_mut() = is_time_up;
        is_time_up
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Δt: {:?}", self.start_time.elapsed())
    }
}
