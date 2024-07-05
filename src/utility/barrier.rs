pub struct Barrier {
    remaining: usize,
}

impl Barrier {
    pub fn new(remaining: usize) -> Self {
        Self { remaining }
    }

    pub fn complete(&mut self) {
        self.remaining = self.remaining.saturating_sub(1);
    }

    pub fn is_complete(&self) -> bool {
        self.remaining == 0
    }
}
