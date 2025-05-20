use std::any::Any;
use crate::interfaces::IProxy;

pub struct Proxy {
    name: String,
    data: Option<Box<dyn Any>>,
}

impl Proxy {
    pub const NAME: &'static str = "Proxy";

    pub fn new(name: Option<String>, data: Option<Box<dyn Any>>) -> Proxy {
        Self { name: name.unwrap_or_else(|| Self::NAME.to_string()), data }
    }
}

impl IProxy for Proxy {
    fn get_proxy_name(&self) -> &str {
        &self.name
    }

    fn get_data(&self) -> Option<&dyn Any> {
        self.data.as_deref()
    }

    fn set_data(&mut self, data: Option<Box<dyn Any>>) {
        self.data = data;
    }

    fn on_register(&mut self) {
        
    }

    fn on_remove(&mut self) {
        
    }
}
