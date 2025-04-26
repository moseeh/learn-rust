pub struct StepIterator<T> {
    beg: T,
    started: bool,
    end: T,
    step: T,
}

use std::ops::Add;
impl<T: Copy + Add<Output = T> + PartialOrd + PartialEq> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            started: false,
            end,
            step,
        }
    }
}

impl<T: PartialEq + Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end || self.beg + self.step > self.end {
            return None;
        }
        if !self.started {
            self.started = true;
            return Some(self.beg);
        }
        self.beg = self.beg + self.step;
        Some(self.beg)
    }
}
