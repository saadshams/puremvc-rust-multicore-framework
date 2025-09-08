use std::sync::{Arc, Mutex};
use crate::{INotification, INotifier};

pub trait ICommand: INotifier {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>);

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync>;
}
