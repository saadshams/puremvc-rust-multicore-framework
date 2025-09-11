use std::any::Any;
use std::sync::{Arc};

pub trait INotification: Any + Send + Sync {
    fn name(&self) -> &str;

    fn body(&self) -> Option<&Arc<dyn Any + Send + Sync>>;
    fn set_body(&mut self, body: Option<Arc<dyn Any + Send + Sync>>);

    fn get_type(&self) -> Option<&str>;
    fn set_type(&mut self, type_: Option<String>);

    fn to_string(&self) -> String;
}
