use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::{IFacade, IMediator, INotification, INotifier};
use crate::patterns::Notifier;

pub struct Mediator {
    notifier: Box<dyn INotifier + Send + Sync>,
    name: String,
    component: Option<Weak<dyn Any + Send + Sync>>,
}

impl Mediator {
    pub const NAME: &'static str = "Mediator";

    pub fn new(name: Option<&str>, component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {
            notifier: Box::new(Notifier::new()),
            name: name.unwrap_or(Self::NAME).into(),
            component
        }
    }
}

impl IMediator for Mediator {
    fn name(&self) -> &str {
        &self.name
    }

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.component.as_ref()
    }

    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.component = component
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

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl INotifier for Mediator {
    fn key(&self) -> &str {
        self.notifier.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.notifier.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.notifier.send_notification(name, body, type_);
    }
}
