pub use std::rc::Rc;

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: Rc<i32>,
    pub max: i32,
}

pub trait Logger {
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
    fn warning(&self, msg: &str);
}
//&ref_cell::Worker
impl<'a> Tracker<'a> {
    //   pub  fn new(logger: &'a mut dyn Logger,num:i32) -> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, num: i32) -> Tracker<'a> {
        Tracker {
            logger,
            value: Rc::new(num),
            max: num,
        }
    }
    pub fn set_value(&self, num: &Rc<usize>) {
        let count = Rc::strong_count(num);
        let percent = (count * 100) / self.max as usize;
        if percent >= 70 && percent < 100 {
            self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    percent
                )
                .as_str(),
            );
        }
        if percent >= 100 {
            self.logger.error("Error: you are over your quota!");
        }
    }
    pub fn peek(&self, num: &Rc<usize>) {
        let count = Rc::strong_count(&num);
        let percent = (count * 100) / self.max as usize;
        self.logger
            .info(format!("Info: you are using up to {}% of your quota", percent).as_str());
    }
}
