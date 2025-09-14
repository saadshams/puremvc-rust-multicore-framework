use std::sync::{Arc, Mutex};
use crate::interfaces::{ICommand, IMediator, INotification, INotifier, IProxy};

pub trait IFacade: INotifier {
    fn initialize_facade(&mut self) {}
    fn initialize_controller(&mut self) {}
    fn initialize_model(&mut self) {}
    fn initialize_view(&mut self) {}

    fn register_command(&self, _notification_name: &str, _factory: fn() -> Box<dyn ICommand + Send + Sync>) {}
    fn has_command(&self, _notification_name: &str) -> bool { false }
    fn remove_command(&self, _notification_name: &str) {}

    fn register_proxy(&self, _proxy: Arc<Mutex<dyn IProxy>>) {}
    fn retrieve_proxy(&self, _proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> { None }
    fn has_proxy(&self, _proxy_name: &str) -> bool { false}
    fn remove_proxy(&self, _proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> { None }

    fn register_mediator(&self, _mediator: Arc<Mutex<dyn IMediator>>) {}
    fn retrieve_mediator(&self, _mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> { None }
    fn has_mediator(&self, _mediator_name: &str) -> bool { false }
    fn remove_mediator(&self, _mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> { None }

    fn notify_observers(&self, _notification: &Arc<dyn INotification>) {}
}
