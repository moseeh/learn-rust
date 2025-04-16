use std::rc::Rc;

/// A logging trait that supports three different levels.
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

/// Tracker keeps track of how many references to a particular value exist.
/// It will use the logger to issue messages when the reference count exceeds
/// various thresholds.
pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    /// Last seen number of references.
    pub value: usize,
    /// Maximum allowed references.
    pub max: usize,
}

impl<'a> Tracker<'a> {
    /// Creates a new Tracker given a logger and a maximum allowed count.
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    /// Updates the current count of references (from the passed Rc) and
    /// logs a message if the current reference count meets certain thresholds.
    pub fn set_value(&mut self, tracked: &Rc<String>) {
        let count = Rc::strong_count(tracked);
        self.value = count;
        let percentage = count * 100 / self.max;
        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            let msg = format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            );
            self.logger.warning(&msg);
        }
    }

    /// Peeks at the current reference count and logs an informational message.
    pub fn peek(&self, tracked: &Rc<String>) {
        let count = Rc::strong_count(tracked);
        let percentage = count * 100 / self.max;
        let msg = format!("you are using up to {}% of your quota", percentage);
        self.logger.info(&msg);
    }
}
