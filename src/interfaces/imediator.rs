use std::any::Any;
use std::sync::{Arc, Weak};
use crate::INotifier;
use crate::interfaces::INotification;

pub trait IMediator: INotifier {
    fn name(&self) -> &str;

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        None
    }

    fn set_component(&mut self, _component: Option<Weak<dyn Any + Send + Sync>>) {

    }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {

    }

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {
        
    }
}
