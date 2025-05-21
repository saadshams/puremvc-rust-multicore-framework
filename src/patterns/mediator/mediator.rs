use std::any::Any;
use crate::IMediator;

pub struct Mediator {
    name: String,
    view: Option<Box<dyn Any>>
}

impl Mediator {
    pub const NAME: &'static str = "Mediator";
    
    pub fn new(name: Option<String>, view: Option<Box<dyn Any>>) -> Mediator {
        Self { name: name.unwrap_or_else(|| Self::NAME.to_string()), view }
    }
}

impl IMediator for Mediator {
    fn get_mediator_name(&self) -> &str {
        &self.name
    }

    fn get_view_component(&self) -> Option<&dyn Any> {
        self.view.as_deref()
    }

    fn set_view(&mut self, view: Option<Box<dyn Any>>) {
        self.view = view;
    }
    
    fn on_register(&mut self) {
        
    }
    
    fn on_remove(&mut self) {
        
    }
}