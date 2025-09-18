use std::sync::{Arc, Mutex};
use crate::interfaces::{ICommand, IMediator, INotification, INotifier, IProxy};

pub trait IFacade: INotifier {
    fn initialize_facade(&self) {}
    fn initialize_controller(&self) {}
    fn initialize_model(&self) {}
    fn initialize_view(&self) {}

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        let _ = notification_name; let _ = factory;
    }

    fn has_command(&self, notification_name: &str) -> bool {
        let _ = notification_name; false
    }

    fn remove_command(&self, notification_name: &str) {
        let _ = notification_name;
    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        let _ = proxy;
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        let _ = proxy_name; None
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        let _ = proxy_name; false
    }

    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        let _ = proxy_name; None
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        let _ = mediator;
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let _ = mediator_name; None
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        let _ = mediator_name; false
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let _ = mediator_name; None
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        let _ = notification;
    }
}
