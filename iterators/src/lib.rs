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
    type Item = u64;

    /// Each call to `next` returns the *current* value in the sequence,
    /// then advances `v` one step along the Collatz rules.
    /// Once we hit 0, we stop.
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            // termination
            None
        } else {
            let current = self.v;
            // advance to the next value
            self.v = match current {
                1 => 0, // when we hit 1, next time we’ll terminate
                even if even % 2 == 0 => even / 2,
                odd => 3 * odd + 1,
            };
            Some(current)
        }
    }
}

/// Return the number of steps required for n to reach 1
/// under the Collatz rules. 0 and 1 both return 0 steps.
pub fn collatz(n: u64) -> usize {
    Collatz::new(n)
        // take all terms *before* the first 1
        .take_while(|&x| x != 1)
        .count()
}
