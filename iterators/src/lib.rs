#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    /// Start a new Collatz sequence at n
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            None
        } else {
            let current = self.v;
            // compute where to go next:
            let next_v = if current == 1 {
                0
            } else if current % 2 == 0 {
                current / 2
            } else {
                3 * current + 1
            };
            // advance internal state
            self.v = next_v;
            // yield a fresh Collatz item holding the *old* value
            Some(Collatz { v: current })
        }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n)
        .take_while(|c| c.v != 1) // compare the v field
        .count()
}
