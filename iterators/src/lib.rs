#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    /// Create a new Collatz sequence starting from `n`.
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }

    /// Inherent `.count()` that returns _steps_ to reach 1
    /// (i.e. number of terms _before_ the 1).
    pub fn count(self) -> usize {
        // take all values before the first 1, and count them
        self.take_while(|c| c.v != 1).count()
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            None
        } else {
            let current = self.v;
            self.v = match current {
                1 => 0, // once we yield 1, end next time
                even if even % 2 == 0 => even / 2,
                odd => 3 * odd + 1,
            };
            Some(Collatz { v: current })
        }
    }
}

/// Free function that also returns the number of steps to 1.
pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}
