use std::any::Any;
use std::sync::{Arc, Mutex, Weak};
use crate::INotifier;
use crate::interfaces::INotification;

pub trait IMediator: Any + INotifier + Send + Sync {
    fn name(&self) -> &str;

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        None
    }

    fn component_mut(&mut self) -> Option<&mut Weak<dyn Any + Send + Sync>> {
        None
    }

    fn set_component(&mut self, _component: Option<Weak<dyn Any + Send + Sync>>) {

    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync>;

    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, _notification: &Arc<Mutex<dyn INotification>>) {

    }

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {
        
    }
}
