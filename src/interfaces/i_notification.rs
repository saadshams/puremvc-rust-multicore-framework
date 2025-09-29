use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{IView, IObserver};

/// The trait definition for a PureMVC MultiCore `INotification`.
///
/// The Observer Pattern as implemented within PureMVC exists to support publish/subscribe
/// communication between actors.
///
/// `INotification`s are not meant to be a replacement for `Event`s, but rather an internal
/// communication mechanism that ensures PureMVC is portable regardless of what type of
/// `Event` mechanism is supported (or not) on a given platform.
///
/// Generally, `IMediator` implementors place `Event` listeners on their view components, and
/// `IProxy` implementors place `Event` listeners on service components. Those `Event`s are
/// then handled in the usual way, and may lead to the broadcast of `INotification`s that
/// trigger `ICommand`s or notify `IMediator`s.
///
/// See [`IView`], [`IObserver`]
pub trait INotification: Any + Send + Sync {
    /// Get the name of the `Notification`.
    ///
    /// # Returns
    /// The name of the `Notification`.
    fn name(&self) -> &str;

    /// Get the body of the `Notification`.
    ///
    /// # Returns
    /// The body of the `Notification`.
    fn body(&self) -> Option<&Arc<dyn Any + Send + Sync>>;

    /// Set the body of the `Notification`.
    ///
    /// # Arguments
    /// * `body` - The body of the `Notification`.
    fn set_body(&mut self, body: Option<Arc<dyn Any + Send + Sync>>);

    /// Get the type of the `Notification`.
    ///
    /// # Returns
    /// The type of the `Notification`.
    fn get_type(&self) -> Option<&str>;

    /// Set the type of the `Notification`.
    ///
    /// # Arguments
    /// * `type_` - The type of the `Notification`.
    fn set_type(&mut self, type_: Option<String>);

    /// Convert the `Notification` to a string representation.
    ///
    /// # Returns
    /// A string representation of the `Notification`.
    fn to_string(&self) -> String;
}
