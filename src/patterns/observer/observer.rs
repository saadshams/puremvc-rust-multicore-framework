use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::interfaces::{IController, IMediator, INotification, IObserver, IView};

/// A base `IObserver` implementation.
///
/// In PureMVC, `IObserver` implementors assume these responsibilities:
///
/// - Encapsulate the notification callback method of the interested object.
/// - Encapsulate the notification context of the interested object.
/// - Provide methods for setting the interested object's notification method and context.
/// - Provide a method for notifying the interested object.
///
/// The Observer Pattern as implemented within PureMVC exists to support publish/subscribe
/// communication between actors.
///
/// An `IObserver` is an object that encapsulates information about an interested object with a
/// notification callback method that should be called when an `INotification` is broadcast. The
/// `IObserver` then acts as a conduit for notifying the interested object.
///
/// `IObserver`s can receive `Notification`s by having their `notify_observer` method invoked,
/// passing in an object implementing the `INotification` interface.
///
/// See [`IView`], [`INotification`]
pub struct Observer {
    /// The notification callback method of the interested object.
    notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>,
    /// The notification context of the interested object.
    context: Option<Arc<dyn Any + Send + Sync>>
}

impl Observer {
    /// Construct a new `Observer` instance.
    ///
    /// The notification callback method should take one parameter of type `INotification`.
    ///
    /// # Arguments
    /// * `notify` - The notification callback method (optional).
    /// * `context` - The context object (optional).
    pub fn new(notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>, context: Option<Arc<dyn Any + Send + Sync>>) -> Self {
        Self {
            notify,
            context,
        }
    }
}

impl IObserver for Observer {
    /// Get the notification callback method.
    ///
    /// # Returns
    /// The notification callback method of the interested object.
    fn notify(&self) -> Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>> {
        self.notify.clone()
    }

    /// Set the notification callback method.
    ///
    /// The notification callback method should take one parameter of type `Notification`.
    ///
    /// # Arguments
    /// * `notify` - The notification callback method of the interested object.
    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>) {
        self.notify = notify;
    }

    /// Get the notification context.
    ///
    /// # Returns
    /// The context object.
    fn context(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        self.context.clone()
    }

    /// Set the notification context.
    ///
    /// # Arguments
    /// * `context` - A reference to the object to be notified.
    fn set_context(&mut self, context: Option<Arc<dyn Any + Send + Sync>>) {
        self.context = context;
    }

    /// Notify the interested object.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to pass to the callback method.
    fn notify_observer(&self, notification: &Arc<dyn INotification>) {
        if let Some(notify) = &self.notify() {
            notify(notification);
        }
    }

    /// Compare a given object to the notification context object.
    ///
    /// # Arguments
    /// * `object` - The object to compare.
    ///
    /// # Returns
    /// `true` if the given object and the notification context are the same, otherwise `false`.
    fn compare_notify_context(&self, object: &Arc<dyn Any + Send + Sync>) -> bool {
        if let Some(context) = self.context() {
            if let (Some(a), Some(b)) = (
                context.downcast_ref::<Arc<dyn IController>>(),
                object.downcast_ref::<Arc<dyn IController>>(),
            ) {
                return Arc::ptr_eq(a, b);
            }

            if let (Some(a), Some(b)) = (
                context.downcast_ref::<Arc<RwLock<dyn IMediator>>>(),
                object.downcast_ref::<Arc<RwLock<dyn IMediator>>>(),
            ) {
                return Arc::ptr_eq(a, b);
            }

            return Arc::ptr_eq(&context, object);
        }

        false
    }
}
