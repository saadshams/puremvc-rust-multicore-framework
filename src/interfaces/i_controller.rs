use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, INotification};

/// The trait definition for a PureMVC MultiCore `IController`.
///
/// In PureMVC, an `IController` implementor follows the 'Command and Controller' strategy, and
/// assumes these responsibilities:
///
/// - Remembering which `ICommand`s are intended to handle which `INotification`s.
/// - Registering itself as an `IObserver` with the `View` for each `INotification` that it has an `ICommand` mapping for.
/// - Creating a new instance of the proper `ICommand` to handle a given `INotification` when notified by the `IView`.
/// - Calling the `ICommand`'s `execute` method, passing in the `INotification`.
///
/// See [`INotification`], [`ICommand`]
pub trait IController: Any + Send + Sync {

    /// Initialize the `IController` Multiton instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the Multiton
    /// instance in your subclass without overriding the constructor.
    fn initialize_controller(&self);

    /// Register a `Notification` to `ICommand` mapping with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to associate the `ICommand` with.
    /// * `factory` - A function that creates a new instance of the `ICommand`.
    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>);

    /// Execute the `ICommand` previously registered as the handler for `Notification`s
    /// with the given notification's name.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to execute the associated `ICommand` for.
    fn execute_command(&self, notification: &Arc<dyn INotification>);

    /// Check if an `ICommand` is registered for a given `Notification` name with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification`.
    ///
    /// # Returns
    /// `true` if an `ICommand` is currently registered for the given `notification_name`, otherwise `false`.
    fn has_command(&self, notification_name: &str) -> bool;

    /// Remove a previously registered `Notification` to `ICommand` mapping from the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to remove the `ICommand` mapping for.
    fn remove_command(&self, notification_name: &str);
}
