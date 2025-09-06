use std::any::Any;

pub trait INotifier {
    fn initialize_notifier(&mut self, key: &str);
    fn send_notification(&self, notification_name: &str, body: Option<Box<dyn Any+ Send + Sync>>, type_: Option<&str>);
}
