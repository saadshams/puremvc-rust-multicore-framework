use std::any::Any;
use std::sync::{Arc, Mutex};

pub trait INotifier {
    fn initialize_notifier(&mut self, key: &str);
    fn send_notification(&self, notification_name: &str, body: Option<Arc<Mutex<dyn Any>>>, type_: Option<&str>);
}
