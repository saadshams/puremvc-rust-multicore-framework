use std::any::Any;
use std::rc::{Rc, Weak};
use crate::{IMediator, INotification};

pub struct Mediator {
    name: String,
    component: Option<Weak<dyn Any>>,
}

impl Mediator {
    pub const NAME: &'static str = "Mediator";

    pub fn new(name: Option<&str>, component: Option<Rc<dyn Any>>) -> Self {
        Self {
            name: name.unwrap_or(Self::NAME).to_string(),
            component: component.map(|rc| Rc::downgrade(&rc)),
        }
    }
}

impl IMediator for Mediator {
    fn name(&self) -> &str {
        &self.name
    }

    fn component(&self) -> Option<Rc<dyn Any>> {
        self.component.as_ref().and_then(|weak| weak.upgrade())
    }

    fn set_component(&mut self, view: Rc<dyn Any>) {
        self.component = Some(Rc::downgrade(&view));
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
