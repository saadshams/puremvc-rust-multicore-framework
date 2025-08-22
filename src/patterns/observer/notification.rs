use std::any::Any;
use crate::INotification;

pub struct Notification {
    name: String,
    body: Option<Box<dyn Any>>,
    type_: Option<String>,
}

impl Notification {
    pub fn new(name: &str, body: Option<Box<dyn Any>>, type_: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            body,
            type_
        }
    }
}

impl INotification for Notification {
    fn name(&self) -> &str {
        &self.name
    }

    fn body(&mut self) -> Option<&mut Box<dyn Any>> {
        self.body.as_mut()
    }

    fn set_body_mut(&mut self, body: Option<Box<dyn Any>>) {
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
            None => "null".to_string()
        };
        let type_ = self.r#type_.as_deref().unwrap_or("null");
        format!("Notification Name: {}\nBody: {}\nType: {}", name, body, type_)
    }
}
