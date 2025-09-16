use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, INotification};

pub trait IController: Any + Send + Sync {
    fn initialize_controller(&self);

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>);
    fn execute_command(&self, notification: &Arc<dyn INotification>);
    fn has_command(&self, notification_name: &str) -> bool;
    fn remove_command(&self, notification_name: &str);
}
