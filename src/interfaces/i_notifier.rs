use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::IFacade;

/// The trait definition for a PureMVC `INotifier`.
///
/// `MacroCommand`, `SimpleCommand`, `Mediator`, and `Proxy` all have a need to send
/// `INotification`s.
///
/// The `INotifier` trait provides a common method called `sendNotification` that relieves
/// implementation code of the necessity to actually construct `INotification`s.
///
/// The `INotifier` trait, which all the above-mentioned types implement, also provides an
/// initialized reference to the `IFacade` Multiton, which is required for the convenience
/// method for sending `INotification`s, but also eases implementation as these types have
/// frequent `IFacade` interactions and usually require access to the `IFacade` anyway.
///
/// See `IFacade`, `INotification`
pub trait INotifier: Any + Send + Sync {
    /// Get the Multiton key for this `INotifier`.
    ///
    /// # Returns
    /// The Multiton key of the `INotifier`.
    fn key(&self) -> &str {
        ""
    }

    /// Get the `IFacade` instance associated with this `INotifier`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade>;

    /// Initialize this `INotifier` instance.
    ///
    /// This is how an `INotifier` gets its Multiton key. Calls to `sendNotification` or
    /// access to the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `INotifier`.
    fn initialize_notifier(&mut self, key: &str);

    /// Send a `Notification`.
    ///
    /// Convenience method to prevent having to construct new `Notification` instances in
    /// implementation code.
    ///
    /// # Arguments
    /// * `name` - The name of the `Notification` to send.
    /// * `body` - The body of the `Notification` (optional).
    /// * `type_` - The type of the `Notification` (optional).
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>);
}
