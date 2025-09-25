use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{IFacade, INotifier};
use crate::patterns::Facade;

/// Error message for uninitialized Multiton key access.
const MULTITON_MSG: &str = "multitonKey for this Notifier not yet initialized!";

/// A base `INotifier` implementation.
///
/// `MacroCommand`, `SimpleCommand`, `Mediator`, and `IProxy` all have a need to send `INotification`s.
///
/// The `INotifier` interface provides a common method called `send_notification` that relieves
/// implementation code of the necessity to actually construct `INotification`s.
///
/// The `Notifier` struct, which all the above-mentioned types extend, provides an initialized
/// reference to an `IFacade` Multiton, which is required by the convenience method for sending
/// `INotification`s, but also eases implementation as these types have frequent `IFacade`
/// interactions and usually require access to the `IFacade` anyway.
///
/// Note: In the MultiCore version of the framework, there is one caveat to `INotifier`s: they
/// cannot send `INotification`s or reach the `IFacade` until they have a valid Multiton key.
///
/// The Multiton key is set:
/// - On an `ICommand` when it is instantiated by the `IController`.
/// - On an `IMediator` when it is registered with the `IView`.
/// - On an `IProxy` when it is registered with the `IModel`.
///
/// See `IProxy`, `IFacade`, `IMediator`, `MacroCommand`, `SimpleCommand`
pub struct Notifier {
    /// The Multiton key for this `Notifier`.
    key: Option<String>
}

impl Notifier {
    /// Construct a new `Notifier` instance.
    pub fn new() -> Self {
        Self {
            key: None
        }
    }
}

impl INotifier for Notifier {
    /// Get the Multiton key for this `Notifier`.
    ///
    /// # Returns
    /// The Multiton key of the `Notifier`.
    fn key(&self) -> &str {
        self.key.as_deref().unwrap_or("")
    }

    /// Get the `IFacade` instance associated with this `Notifier`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    ///
    /// # Panics
    /// If the Multiton key is not initialized (i.e., `initialize_notifier` has not been called).
    fn facade(&self) -> Arc<dyn IFacade> {
        let key = self.key.as_ref().expect(MULTITON_MSG);
        Facade::get_instance(key, |k| Facade::new(k))
    }

    /// Initialize this `Notifier` instance.
    ///
    /// This is how a `Notifier` gets its Multiton key. Calls to `send_notification` or access to
    /// the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `Notifier`.
    fn initialize_notifier(&mut self, key: &str) {
        self.key = Some(key.into());
    }

    /// Send a `Notification`.
    ///
    /// Convenience method to prevent having to construct new `Notification` instances in
    /// implementation code.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to send.
    /// * `body` - The body of the `Notification` (optional).
    /// * `type_` - The type of the `Notification` (optional).
    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.facade().send_notification(notification_name, body, type_);
    }
}
