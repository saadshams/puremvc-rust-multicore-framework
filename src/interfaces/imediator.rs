use std::any::Any;
use std::sync::{Weak};
use crate::interfaces::INotification;

pub trait IMediator: Send {
    fn name(&self) -> &str;

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>>;
    fn component_mut(&mut self) -> Option<&mut Weak<dyn Any + Send + Sync>>;
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>);

    fn list_notification_interests(&self) -> Vec<String>;
    fn handle_notification(&mut self, notification: &dyn INotification);

    fn on_register(&mut self);
    fn on_remove(&mut self);
}
