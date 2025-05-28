use crate::INotification;

pub trait ICommand {
    fn execute(&mut self, notification: &mut dyn INotification);
}
