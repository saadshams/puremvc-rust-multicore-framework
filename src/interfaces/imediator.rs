use std::any::Any;
use crate::interfaces::INotification;

pub trait IMediator {
    fn get_mediator_name(&self) -> &str;
    fn get_view_component(&self) -> Option<&dyn Any>;
    fn set_view(&mut self, view: Option<Box<dyn Any>>);
    fn list_notification_interests(&mut self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
