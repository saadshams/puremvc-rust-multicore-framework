use std::sync::{Arc};
use crate::{INotification, INotifier};

pub trait ICommand: INotifier {
    fn execute(&mut self, notification: &Arc<dyn INotification>);

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync>;
}
