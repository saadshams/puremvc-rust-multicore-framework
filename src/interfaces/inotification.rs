use std::any::Any;
use std::sync::{Arc, Mutex};

pub trait INotification {
    fn name(&self) -> &str;

    fn body(&self) -> Option<&Arc<Mutex<dyn Any + Send + Sync>>>;
    fn set_body(&mut self, body: Option<Arc<Mutex<dyn Any+ Send + Sync>>>);

    fn get_type(&self) -> Option<&str>;
    fn set_type(&mut self, type_: Option<String>);

    fn to_string(&self) -> String;
}
