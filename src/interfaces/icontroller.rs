use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{ICommand, INotification};

pub trait IController: Any + Send + Sync {
    fn execute_command(&self, notification: &Arc<Mutex<dyn INotification>>);
    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>);
    fn has_command(&self, notification_name: &str) -> bool;
    fn remove_command(&self, notification_name: &str);
}
