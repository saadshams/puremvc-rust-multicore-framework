use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::{IFacade, IMediator, INotification, INotifier, IView};
use crate::patterns::Notifier;

/// A base `IMediator` implementation.
///
/// In PureMVC, a `IMediator` implementor assumes these responsibilities:
///
/// - Implement a common method which returns a list of all `INotification`s the `IMediator` has interest in.
/// - Implement a notification callback method for handling `INotification`s.
/// - Implement methods that are called when the `IMediator` is registered or removed from an `IView`.
///
/// Additionally, `IMediator`s typically:
///
/// - Act as an intermediary between one or more view components and the rest of the application.
/// - Place event listeners on view components, and implement handlers which often send `INotification`s or interact with `IProxy`s to post or retrieve data.
/// - Receive `INotification`s, typically containing data, and update view components in response.
///
/// When a `IMediator` is registered with the `IView`, the `list_notification_interests` method is called.
/// The `IMediator` will return a list of `INotification` names which it wishes to be notified about.
///
/// The `IView` will then create an `Observer` object encapsulating that `IMediator`'s `handle_notification`
/// method and register the `Observer` for each `INotification` name returned by the `IMediator`'s
/// `list_notification_interests` method.
///
/// See [`INotification`], [`IView`]
pub struct Mediator {
    /// The underlying `INotifier` instance used for notification functionality.
    notifier: Box<dyn INotifier + Send + Sync>,
    /// The `Mediator`'s name.
    name: String,
    /// The view component associated with this `Mediator`.
    component: Option<Weak<dyn Any + Send + Sync>>,
}

impl Mediator {
    /// The default name for a `Mediator` instance.
    pub const NAME: &'static str = "Mediator";

    /// Construct a new `Mediator` instance.
    ///
    /// # Arguments
    /// * `name` - The name this `Mediator` will be registered with (optional, defaults to `NAME`).
    /// * `component` - The view component (optional).
    pub fn new(name: Option<&str>, component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {
            notifier: Box::new(Notifier::new()),
            name: name.unwrap_or(Self::NAME).into(),
            component
        }
    }
}

impl IMediator for Mediator {
    /// Get the `Mediator` instance's name.
    ///
    /// # Returns
    /// The name of the `Mediator` instance.
    fn name(&self) -> &str {
        &self.name
    }

    /// Get the `Mediator`'s view component.
    ///
    /// # Returns
    /// The view component associated with the `Mediator`.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.component.as_ref()
    }

    /// Set the `Mediator`'s view component.
    ///
    /// # Arguments
    /// * `component` - The view component.
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.component = component
    }

    /// List `Notification` interests.
    ///
    /// # Returns
    /// A list of the `Notification` names this `Mediator` has an interest in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    /// Handle a `Notification`.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to be handled.
    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {

    }

    /// Called by the `IView` when the `Mediator` is registered.
    fn on_register(&mut self) {

    }

    /// Called by the `IView` when the `Mediator` is removed.
    fn on_remove(&mut self) {

    }

    /// Get the `Mediator` as a dynamic `Any` type.
    ///
    /// # Returns
    /// The `Mediator` instance as a mutable `Any` reference.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl INotifier for Mediator {
    /// Get the Multiton key for this `Mediator`.
    ///
    /// # Returns
    /// The Multiton key of the `Mediator`.
    fn key(&self) -> &str {
        self.notifier.key()
    }

    /// Get the `IFacade` instance associated with this `Mediator`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    /// Initialize this `Mediator` instance.
    ///
    /// This is how a `Mediator` gets its Multiton key. Calls to `send_notification` or access to
    /// the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `Mediator`.
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
