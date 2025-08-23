use std::any::Any;
use std::rc::Rc;
use crate::interfaces::INotification;

pub trait IMediator {
    fn name(&self) -> &str;
    fn component(&self) -> Option<Rc<dyn Any>>;
    fn set_component(&mut self, view: Rc<dyn Any>);
    fn list_notification_interests(&mut self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
