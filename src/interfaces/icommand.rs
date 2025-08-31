use std::sync::{Arc, Mutex};
use crate::INotification;

pub trait ICommand: Sync + Send + 'static {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>);
}
