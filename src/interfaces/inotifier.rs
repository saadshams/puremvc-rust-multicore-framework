use std::any::Any;
use std::sync::{Arc, Mutex, Weak};
use crate::interfaces::IFacade;

pub trait INotifier {
    // Weak to avoid Cyclic ref
    // 1) Facade -> Model -> Proxy -> Notifier -> Facade
    // 2) Facade -> Controller -> Commands -> Notifier -> Facade
    // 3) Facade -> View -> Mediator -> Notifier -> Facade
    fn facade(&self) -> Option<Weak<dyn IFacade>> {
        None
    }

    fn initialize_notifier(&mut self, _key: &str) {

    }

    fn send_notification(&self, _notification_name: &str, _body: Option<Arc<Mutex<dyn Any+ Send + Sync>>>, _type_: Option<&str>) {

    }
}
