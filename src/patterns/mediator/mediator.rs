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

    fn component(&self) -> Option<Weak<dyn Any + Send + Sync>> {
        self.component.clone()
    }

    fn set_component(&mut self, component: Weak<dyn Any + Send + Sync>) {
        self.component = Some(component);
    }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, _notification: &dyn INotification) {
        
    }

    fn on_register(&self) {
        // component: RefCell<u32>,
        // *self.component.borrow_mut() += 1; // mutate safely inside RefCell
    }
    
    fn on_remove(&self) {
        
    }
}
