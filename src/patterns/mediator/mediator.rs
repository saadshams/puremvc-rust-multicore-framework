use std::any::Any;
use crate::{IMediator, INotification};

pub struct Mediator {
    name: String,
    view: Option<Box<dyn Any>>
}

impl Mediator {
    pub const NAME: &'static str = "Mediator";
    
    pub fn new(name: Option<String>, view: Option<Box<dyn Any>>) -> Self {
        Self {
            name: name.unwrap_or_else(|| Self::NAME.to_string()),
            view
        }
    }
}

impl IMediator for Mediator {
    fn name(&self) -> &str {
        &self.name
    }

    fn component(&self) -> Option<&dyn Any> {
        self.view.as_deref()
    }

    fn component_mut(&mut self) -> &mut Option<Box<dyn Any>> {
        &mut self.view
    }

    fn set_component_mut(&mut self, view: Option<Box<dyn Any>>) {
        self.view = view;
    }

    fn list_notification_interests(&mut self) -> Vec<String> {
        vec![]
    }

    fn handle_notification(&mut self, _notification: &dyn INotification) {
        
    }

    fn on_register(&mut self) {
        
    }
    
    fn on_remove(&mut self) {
        
    }
}
