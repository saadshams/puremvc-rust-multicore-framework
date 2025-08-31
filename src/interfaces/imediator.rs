use std::any::Any;
use std::sync::Arc;
use crate::interfaces::INotification;

pub trait IMediator {
    fn name(&self) -> &str;
    fn component(&self) -> Option<Arc<dyn Any>>;
    fn set_component(&mut self, view: Arc<dyn Any>);
    fn list_notification_interests(&mut self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
