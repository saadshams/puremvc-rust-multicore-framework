use crate::INotification;

pub trait ICommand: Sync + Send + 'static {
    fn execute(&mut self, notification: &mut dyn INotification);
}
