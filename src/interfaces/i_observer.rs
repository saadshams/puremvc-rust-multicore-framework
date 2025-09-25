use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::INotification;

/// The trait definition for a PureMVC MultiCore `IObserver`.
///
/// In PureMVC, an `IObserver` implementor assumes these responsibilities:
///
/// - Encapsulate the notification callback method of the interested object.
/// - Encapsulate the notification context of the interested object.
/// - Provide methods for setting the interested object's notification method and context.
/// - Provide a method for notifying the interested object.
///
/// The Observer Pattern as implemented within PureMVC exists to support publish/subscribe
/// communication between actors.
///
/// An `IObserver` is an object that encapsulates information about an interested object with
/// a notification callback method that should be called when an `INotification` is broadcast.
/// The `IObserver` then acts as a conduit for notifying the interested object.
///
/// `IObserver`s can receive `INotification`s by having their `notifyObserver` method invoked,
/// passing in an object implementing the `INotification` interface.
///
/// See `IView`, `INotification`
pub trait IObserver: Any + Send + Sync {
    /// Get the notification callback method.
    ///
    /// # Returns
    /// The notification callback method of the interested object.
    fn notify(&self) -> Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>;

    /// Set the notification callback method.
    ///
    /// The notification method should take one parameter of type `Notification`.
    ///
    /// # Arguments
    /// * `notify` - The notification callback method of the interested object.
    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>);

    /// Get the notification context.
    ///
    /// # Returns
    /// The context of the interested object.
    fn context(&self) -> Option<Arc<dyn Any + Send + Sync>>;

    /// Set the notification context.
    ///
    /// # Arguments
    /// * `context` - A reference to the object to be notified.
    fn set_context(&mut self, context: Option<Arc<dyn Any + Send + Sync>>);

    /// Notify the interested object.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to pass to the interested object's notification method.
    fn notify_observer(&self, notification: &Arc<dyn INotification>);

    /// Compare a given object to the notification context.
    ///
    /// # Arguments
    /// * `object` - The object to compare.
    ///
    /// # Returns
    /// `true` if the given object and the notification context are the same, otherwise `false`.
    fn compare_notify_context(&self, object: &Arc<dyn Any + Send + Sync>) -> bool;
}
