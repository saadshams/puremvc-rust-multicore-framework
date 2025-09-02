use std::any::Any;
use std::sync::{Weak};
use crate::interfaces::INotification;

pub trait IMediator: Any + Send + Sync + 'static {
    fn name(&self) -> &str;

    fn component(&self) -> Option<Weak<dyn Any + Send + Sync>>;
    fn set_component(&mut self, component: Weak<dyn Any + Send + Sync>);

    fn list_notification_interests(&self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);
    fn on_register(&self);
    fn on_remove(&self);
}
