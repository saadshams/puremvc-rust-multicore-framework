use std::sync::{Arc};
use crate::{INotification, INotifier, Notifier};
use crate::interfaces::ICommand;

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
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.notifier.as_mut()
    }
}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {

    }
}
