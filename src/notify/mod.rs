use crate::reminder::Reminder;
use std::error::Error;

pub trait Notify {
    fn notify(&self, reminder: &Reminder) -> Result<&Self, Box<dyn Error>>;
}
