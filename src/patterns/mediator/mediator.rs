use std::any::Any;
use std::sync::{Weak};
use crate::{IMediator, INotification};

pub struct Mediator {
    name: String,
    component: Option<Weak<dyn Any + Send + Sync>>,
}

impl Mediator {
    pub const NAME: &'static str = "Mediator";

    pub fn new(name: Option<&str>, component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {
            name: name.unwrap_or(Self::NAME).to_string(),
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

    fn component_mut(&mut self) -> Option<&mut Weak<dyn Any + Send + Sync>> {
        self.component.as_mut()
    }

    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.component = component
    }
    
    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, _notification: &dyn INotification) {
        
    }

    fn on_register(&mut self) {

    }
    
    fn on_remove(&mut self) {
        
    }
}
