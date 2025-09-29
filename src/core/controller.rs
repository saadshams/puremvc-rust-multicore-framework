use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock, Weak};
use crate::core::View;
use crate::interfaces::{ICommand, IController, INotification, IView};
use crate::patterns::Observer;

static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

/// A PureMVC MultiCore `IController` implementation.
///
/// In PureMVC, an `IController` implementor follows the 'Command and Controller' strategy, and
/// assumes these responsibilities:
///
/// - Remembering which `ICommand`s are intended to handle which `INotification`s.
/// - Registering itself as an `Observer` with the View for each `INotification` that it has an `ICommand` mapping for.
/// - Creating a new instance of the proper `ICommand` to handle a given `INotification` when notified by the `IView`.
/// - Calling the ICommand's `execute` method, passing in the INotification.
///
/// See `INotification`, `ICommand`
pub struct Controller {
    /// The Multiton Key for this Core
    key: String,
    /// Local reference to this core's IView
    view: Weak<dyn IView>,
    /// Mapping of `Notification` names to Command factory functions
    command_map: RwLock<HashMap<String, fn() -> Box<dyn ICommand + Send + Sync>>>
}

impl Controller {
    /// Constructor.
    ///
    /// This `IController` implementation is a Multiton, so you should not call the constructor directly,
    /// but instead call the static `Controller::get_instance` method.
    ///
    /// Panics with `crate::core::controller::MultitonErrorControllerExists` if an instance for this Multiton key has already been constructed.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            view: Arc::downgrade(&(View::get_instance(&key, |k| View::new(k)))),
            command_map: RwLock::new(HashMap::new()),
        }
    }

    /// IController Multiton Factory method.
    ///
    /// Returns the `IController` Multiton instance for the specified key.
    pub fn get_instance<T: IController>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IController> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_controller();
                Arc::new(instance)
            })
            .clone()
    }

    /// Remove an `IController` instance.
    ///
    /// # Arguments
    /// * `key` - Multiton key of the `IController` instance to remove
    pub fn remove_controller(key: &str) {
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IController for Controller {
    /// Initialize the `IController` Multiton instance.
    ///
    /// Called automatically by the constructor.
    ///
    /// Note that if you are using a custom `IView` implementor in your application,
    /// you should also subclass `Controller` and override the `IController::initialize_controller` method,
    /// setting `view` equal to the return value of a call to `get_instance` on your IView implementor.
    fn initialize_controller(&self) {

    }

    /// Register an INotification to `ICommand` mapping with the Controller.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the INotification to associate the `ICommand` with.
    /// * `factory` - A function that creates a new instance of the `ICommand`.
    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.command_map.write().ok()
            .and_then(|mut map| {
                if !map.contains_key(notification_name) && let Some(view) = self.view.upgrade() {
                    let context = Controller::get_instance(&self.key, |k| Controller::new(k));
                    let notify = {
                        let controller = Arc::clone(&context);
                        Arc::new(move |notification: &Arc<dyn INotification>| {
                            controller.execute_command(&notification);
                        })
                    };

                    let observer = Observer::new(Some(notify), Some(Arc::new(context)));
                    view.register_observer(notification_name, Arc::new(observer));
                }
                map.insert(notification_name.into(), factory)
            });
    }

    /// Execute the `ICommand` previously registered as the handler for `INotifications`
    /// with the given notification's name.
    ///
    /// # Arguments
    /// * `notification` - The `INotification` to execute the associated `ICommand` for
    fn execute_command(&self, notification: &Arc<dyn INotification>) {
        self.command_map.read().ok()
            .and_then(|map| map.get(notification.name()).cloned())
            .map(|factory| {
                let mut command = factory();
                command.initialize_notifier(&self.key);
                command.execute(notification);
            });
    }

    /// Check if an `ICommand` is registered for a given `INotification` name with the `Controller`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `INotification`.
    ///
    /// Returns `true` if an `ICommand` is currently registered for the given `notification_name`, otherwise `false`.
    fn has_command(&self, notification_name: &str) -> bool {
        self.command_map.read().ok()
            .map(|map| map.contains_key(notification_name))
            .unwrap_or(false)
    }

    /// Remove a previously registered `Notification` to `ICommand` mapping from the `Controller`.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the `INotification` to remove the `ICommand` mapping for.
    fn remove_command(&self, notification_name: &str) {
        self.command_map.write().ok()
            .and_then(|mut map| map.remove(notification_name))
            .and_then(|_| self.view.upgrade())
            .map(|view| {
                let context = Controller::get_instance(&self.key, |k| Controller::new(k));
                view.remove_observer(notification_name, Arc::new(context));
            });
    }
}