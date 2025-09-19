use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::{INotification, INotifier};

pub trait IMediator: INotifier {
    fn name(&self) -> &str;

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>>;

    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>);

    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        let _ = notification;
    }

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {
        
    }
}
