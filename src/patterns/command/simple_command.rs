use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, IFacade, INotification, INotifier};
use crate::patterns::Notifier;

pub struct SimpleCommand {
    notifier: Box<dyn INotifier + Send + Sync>,
}

impl SimpleCommand {
    pub fn new() -> Self {
        Self {
            notifier: Box::new(Notifier::new())
        }
    }
}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {

    }
}

impl INotifier for SimpleCommand {
    fn key(&self) -> &str {
        self.notifier.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.notifier.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.notifier.send_notification(name, body, type_);
    }
}
