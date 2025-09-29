use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::{INotification, INotifier, IView};

/// The trait definition for a PureMVC MultiCore `IMediator`.
///
/// In PureMVC, an `IMediator` implementor assumes these responsibilities:
///
/// - Implement a common method which returns a list of all `INotification`s the `IMediator` has interest in.
/// - Implement a notification callback method for handling `INotification`s.
/// - Implement methods that are called when the `IMediator` is registered or removed from an `IView`.
///
/// Additionally, `IMediator`s typically:
///
/// - Act as an intermediary between one or more view components and the rest of the application.
/// - Place `Event` listeners on view components, and implement handlers which often send `INotification`s or interact with `IProxy`s to post or retrieve data.
/// - Receive `INotification`s, typically containing data, and update view components in response.
///
/// When an `IMediator` is registered with the `IView`, the `IMediator`'s `listNotificationInterests` method is called.
/// The `IMediator` will return a list of `INotification` names which it wishes to be notified about.
///
/// The `IView` will then create an `IObserver` object encapsulating that `IMediator`'s `handleNotification` method
/// and register the `IObserver` for each `INotification` name returned by the `IMediator`'s `listNotificationInterests` method.
///
/// See [`INotification`], [`IView`]
pub trait IMediator: INotifier {
    /// Get the `IMediator` instance's name.
    ///
    /// # Returns
    /// The name of the `IMediator` instance.
    fn name(&self) -> &str;

    /// Get the `IMediator`'s component.
    ///
    /// # Returns
    /// The view component associated with the `IMediator`.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>>;

    /// Set the `IMediator`'s component.
    ///
    /// # Arguments
    /// * `component` - The view component to associate with the `IMediator`.
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>);

    /// List `Notification` interests.
    ///
    /// # Returns
    /// A list of the `Notification` names this `IMediator` has an interest in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![]
    }

    /// Handle a `Notification`.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to be handled.
    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        let _ = notification;
    }

    /// Called by the `IView` when the `IMediator` is registered.
    fn on_register(&mut self) {

    }

    /// Called by the `IView` when the `IMediator` is removed.
    fn on_remove(&mut self) {

    }

    /// Get the `IMediator` as a dynamic `Any` type.
    ///
    /// # Returns
    /// The `IMediator` instance as a mutable `Any` reference.
    fn as_any(&mut self) -> &mut dyn Any;
}
