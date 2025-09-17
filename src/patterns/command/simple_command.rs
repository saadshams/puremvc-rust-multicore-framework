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

impl INotifier for SimpleCommand {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        Some(self.notifier.as_mut())
    }

    fn facade(&self) -> Option<Arc<dyn IFacade>> {
        self.notifier.facade()
    }

    fn send_notification(&self, _notification_name: &str, _body: Option<Arc<dyn Any + Send + Sync>>, _type_: Option<&str>) {
        self.notifier.send_notification(_notification_name, _body, _type_);
    }
}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {

    }
}
