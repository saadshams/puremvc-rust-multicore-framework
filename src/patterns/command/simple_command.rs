use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, IFacade, INotification, INotifier};
use crate::patterns::Notifier;

/// A base `ICommand` implementation for executing a block of business logic.
///
/// Your implementation should override the `execute` method where your business logic will
/// handle the `INotification`.
///
/// See `ICommand`, `IController`, `INotification`, `MacroCommand`, `INotifier`
pub struct SimpleCommand {
    /// The underlying `INotifier` instance used for notification functionality.
    notifier: Box<dyn INotifier + Send + Sync>,
}

impl SimpleCommand {
    /// Construct a new `SimpleCommand`.
    ///
    /// Creates a new instance with a default `Notifier` for handling notifications.
    pub fn new() -> Self {
        Self {
            notifier: Box::new(Notifier::new())
        }
    }
}

impl ICommand for SimpleCommand {
    /// Respond to the `Notification` that triggered this `Command`.
    ///
    /// Perform business logic, such as complex validation, processing, or model changes.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` object that triggered the execution of this `Command`.
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {

    }
}

impl INotifier for SimpleCommand {
    /// Get the Multiton key for this `SimpleCommand`.
    ///
    /// # Returns
    /// The Multiton key of the `SimpleCommand`.
    fn key(&self) -> &str {
        self.notifier.key()
    }

    /// Get the `IFacade` instance associated with this `SimpleCommand`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    /// Initialize this `SimpleCommand` instance.
    ///
    /// This is how a `SimpleCommand` gets its Multiton key. Calls to `send_notification` or
    /// access to the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `SimpleCommand`.
    fn initialize_notifier(&mut self, key: &str) {
        self.notifier.initialize_notifier(key);
    }

    /// Send a `Notification`.
    ///
    /// Convenience method to prevent having to construct new `Notification` instances in
    /// implementation code.
    ///
    /// # Arguments
    /// * `name` - The name of the `Notification` to send.
    /// * `body` - The body of the `Notification` (optional).
    /// * `type_` - The type of the `Notification` (optional).
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.notifier.send_notification(name, body, type_);
    }
}
