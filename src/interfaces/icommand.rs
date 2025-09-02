use std::sync::{Arc, Mutex};
use crate::INotification;

pub trait ICommand: Send + Sync {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>);
}
