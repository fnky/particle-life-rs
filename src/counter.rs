#[derive(Debug, Default)]
pub struct BoundedCounter {
    pub current: usize,
    pub lower: usize,
    pub upper: usize,
    pub continious: bool,
}

impl BoundedCounter {
    pub fn increment(&mut self) -> usize {
        if self.current >= self.upper {
            self.current = if self.continious {
                self.lower
            } else {
                self.upper
            };
        } else {
            self.current += 1;
        }

        self.current
    }

    pub fn decrement(&mut self) -> usize {
        if self.current <= self.lower {
            self.current = if self.continious {
                self.upper
            } else {
                self.lower
            };
        } else {
            self.current -= 1;
        }

        self.current
    }

    pub fn current(&self) -> usize {
        self.current
    }
}
