use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use crate::core::{Controller, Model, View};
use crate::interfaces::{ICommand, IController, IFacade, IMediator, IModel, INotification, INotifier, IProxy, IView};
use crate::patterns::Notification;

/// The Multiton instance map for `Facade` instances.
static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IFacade>>>> = LazyLock::new(|| Default::default());

/// A base Multiton `IFacade` implementation.
///
/// The Facade Pattern suggests providing a single struct to act as a central point of
/// communication for a subsystem.
///
/// In PureMVC, the `IFacade` acts as an interface between the core MVC actors `IModel`,
/// `IView`, `IController`, and the rest of your application, which (aside from view components
/// and data objects) is mostly expressed with `ICommand`s, `IMediator`s, and `IProxy`s.
///
/// This means you don't need to communicate with the `IModel`, `IView`, or `IController`
/// instances directly; you can just go through the `IFacade`. Conveniently, `ICommand`s,
/// `IMediator`s, and `IProxy`s all have a built-in reference to their `IFacade` after
/// initialization, so they're all plugged in and ready to communicate with each other.
///
/// See [`Model`], [`View`], [`Controller`], [`INotification`], [`ICommand`], [`IMediator`], [`IProxy`]
pub struct Facade {
    /// The Multiton key for this `Facade` instance.
    key: String,
    /// The `IController` instance for this core.
    controller: Arc<dyn IController>,
    /// The `IModel` instance for this core.
    model: Arc<dyn IModel>,
    /// The `IView` instance for this core.
    view: Arc<dyn IView>
}

impl Facade {
    /// Construct a new `Facade` instance.
    ///
    /// This `IFacade` implementation is a Multiton, so you should not call this constructor
    /// directly, but instead call the static `get_instance` method.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `Facade`.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            controller: Controller::get_instance(key, |k| Controller::new(k)),
            model: Model::get_instance(key, |k| Model::new(k)),
            view: View::get_instance(key, |k| View::new(k))
        }
    }

    /// Get or create a Multiton `IFacade` instance for the specified key.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for the `IFacade`.
    /// * `factory` - A function that creates a new `IFacade` instance for the given key.
    ///
    /// # Returns
    /// The `IFacade` Multiton instance for the specified key.
    pub fn get_instance<T: IFacade>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IFacade> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_facade();
                Arc::new(instance)
            })
            .clone()
    }

    /// Check if a Core is registered.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for the Core.
    ///
    /// # Returns
    /// `true` if a Core is registered with the given `key`, otherwise `false`.
    pub fn has_core(key: &str) -> bool {
        INSTANCE_MAP.read().unwrap().contains_key(key)
    }

    /// Remove a Core.
    ///
    /// Removes the `IModel`, `IView`, `IController`, and `IFacade` instances for the given key.
    ///
    /// # Arguments
    /// * `key` - The Multiton key of the Core to remove.
    pub fn remove_core(key: &str) {
        Model::remove_model(key);
        View::remove_view(key);
        Controller::remove_controller(key);
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IFacade for Facade {
    /// Initialize the Multiton `Facade` instance.
    ///
    /// Called automatically by the `get_instance` method. Override in your implementation to
    /// perform any subclass-specific initializations. Be sure to call `initialize_facade` from
    /// the parent implementation if overridden.
    fn initialize_facade(&self) {
        self.initialize_model();
        self.initialize_controller();
        self.initialize_view();
    }

    /// Initialize the `IController`.
    ///
    /// Called by the `initialize_facade` method. Override this method in an implementation of
    /// `Facade` if you want to provide a different `IController`.
    fn initialize_controller(&self) {

    }

    /// Initialize the `IModel`.
    ///
    /// Called by the `initialize_facade` method. Override this method in an implementation of
    /// `Facade` if you wish to initialize a different `IModel`.
    fn initialize_model(&self) {

    }

    /// Initialize the `IView`.
    ///
    /// Called by the `initialize_facade` method. Override this method in an implementation of
    /// `Facade` if you wish to initialize a different `IView`.
    fn initialize_view(&self) {

    }

    /// Register a `Notification` to `Command` mapping with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to associate the `Command` with.
    /// * `factory` - A function that creates a new instance of a `Command`.
    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.controller.register_command(notification_name, factory);
    }

    /// Check if a `Command` is registered for a given `Notification` name with the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification`.
    ///
    /// # Returns
    /// `true` if a `Command` is currently registered for the given `notification_name`, otherwise `false`.
    fn has_command(&self, notification_name: &str) -> bool {
        self.controller.has_command(notification_name)
    }

    /// Remove a previously registered `Notification` to `Command` mapping from the `IController`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `Notification` to remove the `Command` mapping for.
    fn remove_command(&self, notification_name: &str) {
        self.controller.remove_command(notification_name);
    }

    /// Register a `Proxy` instance with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy` - An object reference to be held by the `IModel`.
    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>) {
        self.model.register_proxy(proxy);
    }

    /// Retrieve a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance to retrieve.
    ///
    /// # Returns
    /// The `Proxy` instance previously registered with the given `proxy_name`.
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.model.retrieve_proxy(proxy_name)
    }

    /// Check if a `Proxy` is registered with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance you're looking for.
    ///
    /// # Returns
    /// `true` if a `Proxy` is currently registered with the given `proxy_name`, otherwise `false`.
    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.model.has_proxy(proxy_name)
    }

    /// Remove a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `name` - Name of the `Proxy` instance to be removed.
    ///
    /// # Returns
    /// The `Proxy` that was removed from the `IModel`.
    fn remove_proxy(&self, name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.model.remove_proxy(name)
    }

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
    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        self.view.register_mediator(mediator);
    }

    /// Retrieve a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` instance to retrieve.
    ///
    /// # Returns
    /// The `Mediator` instance previously registered in this core with the given `mediator_name`.
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.retrieve_mediator(mediator_name)
    }

    /// Check if a `Mediator` is registered with the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the `Mediator` instance you're looking for.
    ///
    /// # Returns
    /// `true` if a `Mediator` is registered in this core with the given `mediator_name`, otherwise `false`.
    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.view.has_mediator(mediator_name)
    }

    /// Remove a `Mediator` from the `IView`.
    ///
    /// # Arguments
    /// * `mediator_name` - Name of the `Mediator` instance to be removed.
    ///
    /// # Returns
    /// The `Mediator` that was removed from this core's `IView`.
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.remove_mediator(mediator_name)
    }

    /// Notify `Observer`s.
    ///
    /// This method allows you to send custom `Notification` instances using the `IFacade`.
    ///
    /// Usually you should just call `send_notification` and pass the parameters, never having
    /// to construct a `Notification` yourself.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` to have the `IView` notify `Observer`s of.
    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.view.notify_observers(notification);
    }
}

impl INotifier for Facade {
    /// Get the Multiton key for this `Facade`.
    ///
    /// # Returns
    /// The Multiton key of the `Facade`.
    fn key(&self) -> &str {
        self.key.as_str()
    }

    /// Get the `IFacade` instance associated with this `Facade`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade> {
        Facade::get_instance(&self.key, |k| Facade::new(k))
    }

    /// Initialize this `Facade` instance.
    ///
    /// This is how a `Facade` gets its Multiton key. Calls to `send_notification` or access to
    /// the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `Facade`.
    fn initialize_notifier(&mut self, key: &str) {
        self.key = key.into();
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
        let notification = Notification::new(notification_name, body, type_);
        self.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    }
}
