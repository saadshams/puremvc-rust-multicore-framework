use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::interfaces::{IMediator, INotification, IObserver};

/// The trait definition for a PureMVC MultiCore `IView`.
///
/// In PureMVC, an `IView` implementor assumes these responsibilities:
///
/// - Maintain a cache of `IMediator` instances.
/// - Provide methods for registering, retrieving, and removing `IMediator`s.
/// - Manage the `IObserver` lists for each `INotification`.
/// - Provide a method for attaching `IObserver`s to an `INotification`'s `IObserver` list.
/// - Provide a method for broadcasting an `INotification` to each of the `IObserver`s in a list.
/// - Notify the `IObserver`s of a given `INotification` when it is broadcast.
///
/// See `IMediator`, `IObserver`, `INotification`
pub trait IView: Any + Sync + Send {
    /// Initialize the `IView` Multiton instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the
    /// Multiton instance in your subclass without overriding the constructor.
    fn initialize_view(&self);

    /// Register an `Observer` to be notified of `Notification`s with a given name.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to notify this `Observer` of.
    /// * `observer` - The `Observer` to register.
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>);

    /// Remove an `Observer` from the list for a given `Notification` name.
    ///
    /// # Arguments
    /// * `notification_name` - The `Notification` list to remove from.
    /// * `context` - Remove `Observer`s with this object as the notification context.
    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>);

    /// Notify the `Observer`s for a particular `Notification`.
    ///
    /// All previously attached `Observer`s for this `Notification`'s list are notified and
    /// are passed a reference to the `Notification` in the order in which they were registered.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to notify `Observer`s of.
    fn notify_observers(&self, notification: &Arc<dyn INotification>);

    /// Register a `Mediator` instance with the `IView`.
    ///
    /// Registers the `Mediator` so that it can be retrieved by name, and interrogates the
    /// `Mediator` for its `Notification` interests.
    ///
    /// If the `Mediator` returns a list of `Notification` names to be notified about, an
    /// `Observer` is created encapsulating the `Mediator` instance's `handleNotification`
    /// method and registering it as an `Observer` for all `Notification`s the `Mediator`
    /// is interested in.
    ///
    /// # Arguments
    /// * `mediator` - A reference to the `Mediator` instance.
    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>);

    /// Retrieve a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` instance to retrieve.
    ///
    /// # Returns
    /// The `Mediator` instance previously registered in this core with the given `mediator_name`.
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>>;

    /// Check if a `Mediator` is registered with the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` you're looking for.
    ///
    /// # Returns
    /// `true` if a `Mediator` is registered in this core with the given `mediator_name`, otherwise `false`.
    fn has_mediator(&self, mediator_name: &str) -> bool;

    /// Remove a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - Name of the `Mediator` instance to be removed.
    ///
    /// # Returns
    /// The `Mediator` that was removed from this core's `IView`.
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>>;
}
