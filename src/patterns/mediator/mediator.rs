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
            name: name.unwrap_or(Self::NAME).to_string(),
            component
        }
    }
}

impl INotifier for Mediator {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        Some(self.notifier.as_mut())
    }

    fn facade(&self) -> Option<Arc<dyn IFacade>> {
        self.notifier.facade()
    }

    fn send_notification(&self, _notification_name: &str, _body: Option<Arc<dyn Any + Send + Sync>>, _type_: Option<&str>) {
        self.notifier.send_notification(_notification_name, _body, _type_);
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
}
