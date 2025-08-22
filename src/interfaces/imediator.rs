use std::any::Any;
use crate::interfaces::INotification;

pub trait IMediator {
    fn name(&self) -> &str;
    fn component(&self) -> Option<&dyn Any>;
    fn component_mut(&mut self) -> &mut Option<Box<dyn Any>>;
    fn set_component_mut(&mut self, view: Option<Box<dyn Any>>);
    fn list_notification_interests(&mut self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
