use std::fmt::{Debug, Display, Formatter, Result};

pub struct Score(usize, usize);

impl Score {
    pub fn new() -> Score {
        Score(0, 0)
    }

    pub fn win(&mut self) {
        self.0 += 1;
    }

    pub fn loose(&mut self) {
        self.1 += 1;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "won: {}, lost: {}", self.0, self.1)
    }
}

impl Debug for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self::Display::fmt(&self, f)
    }
}
