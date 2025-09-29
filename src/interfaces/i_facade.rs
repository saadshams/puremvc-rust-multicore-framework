use std::sync::{Arc, RwLock};
use crate::interfaces::{ICommand, IMediator, INotification, INotifier, IProxy, IModel, IView, IController};

/// The trait definition for a PureMVC MultiCore `IFacade`.
///
/// The Facade Pattern suggests providing a single class to act as a central point of communication
/// for a subsystem.
///
/// In PureMVC, an `IFacade` acts as an interface between the core MVC actors `IModel`, `IView`,
/// `IController`, and the rest of your application, which (aside from view components and data
/// objects) is mostly expressed with `ICommand`s, `IMediator`s, and `IProxy`s.
///
/// This means you don't need to communicate with the `IModel`, `IView`, `IController` instances
/// directly; you can just go through the `IFacade`. And conveniently, `ICommand`s, `IMediator`s,
/// and `IProxy`s all have a built-in reference to their `IFacade` after initialization, so they're
/// all plugged in and ready to communicate with each other.
///
/// See [`IModel`], [`IView`], [`IController`], [`IProxy`], [`IMediator`], [`ICommand`], [`INotification`]
pub trait IFacade: INotifier {
    /// Initialize the `IFacade` Multiton instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the Multiton
    /// instance in your subclass without overriding the constructor.
    fn initialize_facade(&self) {}

    /// Initialize the `IController` instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the
    /// `IController` instance in your subsystem.
    fn initialize_controller(&self) {}

    /// Initialize the `IModel` instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the
    /// `IModel` instance in your subsystem.
    fn initialize_model(&self) {}

    /// Initialize the `IView` instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the
    /// `IView` instance in your subsystem.
    fn initialize_view(&self) {}

    /// Register a `Notification` to `ICommand` mapping with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to associate the `ICommand` with.
    /// * `factory` - A function that creates a new instance of the `ICommand`.
    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        let _ = notification_name; let _ = factory;
    }

    /// Check if an `ICommand` is registered for a given `Notification` name with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification`.
    ///
    /// # Returns
    /// `true` if an `ICommand` is currently registered for the given `notification_name`, otherwise `false`.
    fn has_command(&self, notification_name: &str) -> bool {
        let _ = notification_name; false
    }

    /// Remove a previously registered `Notification` to `ICommand` mapping from the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to remove the `ICommand` mapping for.
    fn remove_command(&self, notification_name: &str) {
        let _ = notification_name;
    }

    /// Register a `Proxy` instance with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy` - An object reference to be held by the `IModel`.
    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>) {
        let _ = proxy;
    }

    /// Retrieve a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance to retrieve.
    ///
    /// # Returns
    /// The `Proxy` instance previously registered with the given `proxy_name`.
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        let _ = proxy_name; None
    }

    /// Check if a `Proxy` is registered with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance you're looking for.
    ///
    /// # Returns
    /// `true` if a `Proxy` is currently registered with the given `proxy_name`, otherwise `false`.
    fn has_proxy(&self, proxy_name: &str) -> bool {
        let _ = proxy_name; false
    }

    /// Remove a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - Name of the `Proxy` instance to be removed.
    ///
    /// # Returns
    /// The `Proxy` that was removed from the `IModel`.
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        let _ = proxy_name; None
    }

    /// Register a `Mediator` instance with the `IView`.
    ///
    /// Registers the `Mediator` so that it can be retrieved by name, and interrogates the
    /// `Mediator` for its `Notification` interests.
    ///
    /// If the `Mediator` returns a list of `Notification` names to be notified about, an
    /// `Observer` is created encapsulating the `Mediator` instance's `handleNotification`
    /// method and registering it as an `IObserver` for all `Notification`s the
    /// `Mediator` is interested in.
    ///
    /// # Arguments
    /// * `mediator` - A reference to the `Mediator` instance.
    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        let _ = mediator;
    }

    /// Retrieve a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` instance to retrieve.
    ///
    /// # Returns
    /// The `Mediator` instance previously registered in this core with the given `mediator_name`.
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        let _ = mediator_name; None
    }

    /// Check if a `Mediator` is registered with the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` you're looking for.
    ///
    /// # Returns
    /// `true` if a `Mediator` is registered in this core with the given `mediator_name`, otherwise `false`.
    fn has_mediator(&self, mediator_name: &str) -> bool {
        let _ = mediator_name; false
    }

    /// Remove a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - Name of the `Mediator` instance to be removed.
    ///
    /// # Returns
    /// The `Mediator` that was removed from this core's `IView`.
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        let _ = mediator_name; None
    }

    /// Notify the `Observer`s for a particular `Notification`.
    ///
    /// This method allows you to send custom `Notification` classes using the `IFacade`.
    ///
    /// Usually you should just call `sendNotification` and pass the parameters,
    /// never having to construct a `Notification` yourself.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to have the `IView` notify `Observer`s of.
    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        let _ = notification;
    }
}
