use std::sync::{Arc};
use crate::interfaces::{INotification, INotifier, IController};

/// The trait definition for a PureMVC MultiCore `ICommand`.
///
/// See [`IController`], [`INotification`]
pub trait ICommand: INotifier {
    /// Execute the `ICommand`'s logic to handle a given `INotification`.
    ///
    /// # Arguments
    /// * `notification` - An `INotification` to handle.
    fn execute(&mut self, notification: &Arc<dyn INotification>);
}
