use std::any::Any;
use std::sync::Arc;
use crate::interfaces::{IFacade, INotifier, IProxy, IModel};
use crate::patterns::Notifier;

/// A base `IProxy` implementation.
///
/// In PureMVC, `IProxy` implementors assume these responsibilities:
///
/// - Implement a common method which returns the name of the `IProxy`.
/// - Provide methods for setting and getting a data object.
///
/// Additionally, `IProxy`s typically:
///
/// - Provide methods for manipulating the data object and referencing it by type.
/// - Generate `INotification`s when their data object changes.
/// - Expose their name as a constant called `NAME`.
/// - Encapsulate interaction with local or remote services used to fetch and persist data.
///
/// See [`IModel`]
pub struct Proxy {
    /// The underlying `INotifier` instance used for notification functionality.
    notifier: Box<dyn INotifier + Send + Sync>,
    /// The `Proxy`'s name.
    name: String,
    /// The `Proxy`'s data object.
    data: Option<Arc<dyn Any + Send + Sync>>
}

impl Proxy {
    /// The default name for a `Proxy` instance.
    pub const NAME: &'static str = "Proxy";

    /// Construct a new `Proxy` instance.
    ///
    /// # Arguments
    /// * `name` - The name this `Proxy` will be registered with (optional, defaults to `NAME`).
    /// * `data` - The data object (optional).
    pub fn new(name: Option<&str>, data: Option<Arc<dyn Any + Send + Sync>>) -> Self {
        Self {
            notifier: Box::new(Notifier::new()),
            name: name.unwrap_or(Self::NAME).into(),
            data
        }
    }
}

impl IProxy for Proxy {
    /// Get the `Proxy` instance's name.
    ///
    /// # Returns
    /// The name of the `Proxy` instance.
    fn name(&self) -> &str {
        &self.name
    }

    /// Get the `Proxy`'s data object.
    ///
    /// # Returns
    /// The data object.
    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.data.as_ref()
    }

    /// Set the `Proxy`'s data object.
    ///
    /// # Arguments
    /// * `data` - The data object this `Proxy` will tend.
    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.data = data;
    }

    /// Called by the `Model` when the `Proxy` is registered.
    fn on_register(&mut self) {

    }

    /// Called by the `Model` when the `Proxy` is removed.
    fn on_remove(&mut self) {

    }

    /// Get the `Proxy` as a dynamic `Any` type.
    ///
    /// # Returns
    /// The `Proxy` instance as a mutable `Any` reference.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl INotifier for Proxy {
    /// Get the Multiton key for this `Proxy`.
    ///
    /// # Returns
    /// The Multiton key of the `Proxy`.
    fn key(&self) -> &str {
        self.notifier.key()
    }

    /// Get the `IFacade` instance associated with this `Proxy`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    /// Initialize this `Proxy` instance.
    ///
    /// This is how a `Proxy` gets its Multiton key. Calls to `send_notification` or access to
    /// the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `Proxy`.
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