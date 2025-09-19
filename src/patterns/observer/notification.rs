use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::INotification;

pub struct Notification {
    name: String,
    body: Option<Arc<dyn Any + Send + Sync>>,
    type_: Option<String>,
}

impl Notification {
    pub fn new(name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) -> Self {
        Self {
            name: name.into(),
            body,
            type_: type_.map(|t| t.into()),
        }
    }
}

impl INotification for Notification {
    fn name(&self) -> &str {
        &self.name
    }

    fn body(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.body.as_ref()
    }
    
    fn set_body(&mut self, body: Option<Arc<dyn Any + Send + Sync>>) {
        self.body = body;
    }

    fn get_type(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    fn set_type(&mut self, type_: Option<String>) {
        self.type_ = type_;
    }
    
    fn to_string(&self) -> String {
        let name = &self.name;
        let body = match &self.body {
            Some(b) => format!("{:?}", b),
            None => "null".into()
        };
        let type_ = self.r#type_.as_deref().unwrap_or("null");
        format!("Notification Name: {}\nBody: {}\nType: {}", name, body, type_)
    }
}
