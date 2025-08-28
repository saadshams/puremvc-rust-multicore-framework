use crate::{ICommand, INotification};

pub trait IController: Sync + Send + 'static {
    fn key(&self) -> &str;
    fn execute_command(&self, notification: &mut dyn INotification);
    fn register_command(&self, notification_name: &str, factory: Box<dyn Fn() -> Box<dyn ICommand> + Send + Sync>);
    fn has_command(&self, notification_name: &str) -> bool;
    fn remove_command(&self, notification_name: &str);
}