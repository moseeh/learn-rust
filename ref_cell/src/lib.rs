pub use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;

pub use messenger::*;

pub mod messenger;

#[derive(Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
pub    fn new(num:usize) -> Worker {
        Worker {
            track_value: Rc::new(num),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
  
}
