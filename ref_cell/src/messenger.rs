// Re-export RefCell from std::cell to allow interior mutability.
pub use std::cell::RefCell;
// Re-export Rc from std::rc for reference-counted pointers.
pub use std::rc::Rc;

// Define a trait Logger with three logging methods.
pub trait Logger {
    fn warning(&self, msg: &str); // Method for warning messages.
    fn info(&self, msg: &str);    // Method for info messages.
    fn error(&self, msg: &str);   // Method for error messages.
}

// Define a generic struct Tracker which logs usage of reference-counted values.
#[derive(Debug, Clone)]
pub struct Tracker<'a, T: Logger> {
    logger: &'a T,               // Logger reference for emitting log messages.
    value: RefCell<usize>,      // Interior mutable field to track current usage count.
    max: usize,                 // Maximum usage limit for triggering warnings/errors.
}

impl<'a, T> Tracker<'a, T>
where
    T: Logger, // Constraint: T must implement Logger trait.
{
    // Constructor method to create a new Tracker instance.
    pub fn new(logger: &T, max: usize) -> Tracker<T> {
        Tracker {
            logger,             // Store reference to logger.
            value: RefCell::new(0), // Initialize internal counter to 0.
            max,                // Set the max threshold.
        }
    }

    // Set the current value count based on the strong reference count of Rc<usize>.
    pub fn set_value(&self, value: &Rc<usize>) {
        self.value.replace(Rc::strong_count(value)); // Replace stored value with current reference count.

        // Calculate how much of the quota has been used in percentage.
        let percentage_of_max = convert_percentage(self.max, Rc::strong_count(&value));

        // If usage is 100% or more, log an error.
        if percentage_of_max >= 100 {
            self.logger.error("Error: you are over your quota!");
            return;
        // If usage is 70% or more, log a warning.
        } else if percentage_of_max >= 70 {
            self.logger
                .warning(&format!("Warning: you have used up over {percentage_of_max}% of your quota! Proceeds with precaution"));
        }
    }

    // Peek current usage and log it as informational message.
    pub fn peek(&self, value: &Rc<usize>) {
        let percentage_of_max = convert_percentage(self.max, Rc::strong_count(&value)); // Calculate current usage.
        self.logger.info(&format!(
            "Info: you are using up to {percentage_of_max}% of your quota" // Log info with current usage percentage.
        ))
    }
}

// Helper function to convert current value to percentage of max.
fn convert_percentage(max: usize, v: usize) -> usize {
    (100 * v) / max // Return v as a percentage of max.
}