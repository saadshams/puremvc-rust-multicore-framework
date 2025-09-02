use std::any::Any;
use crate::IProxy;

pub struct Proxy {
    name: String,
    data: Option<Box<dyn Any + Send + Sync>>
}

impl Proxy {
    pub const NAME: &'static str = "Proxy";

    pub fn new(name: Option<&str>, data: Option<Box<dyn Any + Send + Sync>>) -> Self {
        Self {
            name: name.unwrap_or(Self::NAME).to_string(),
            data
        }
    }
}

impl IProxy for Proxy {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.data.as_deref()
    }

    fn data_mut(&mut self) -> Option<&mut (dyn Any + Send + Sync)> {
        self.data.as_deref_mut()
    }

    fn set_data(&mut self, data: Option<Box<dyn Any + Send + Sync>>) {
        self.data = data;
    }


    fn on_register(&mut self) {
        
    }

    fn on_remove(&mut self) {
        
    }
}
