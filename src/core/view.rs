use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use crate::interfaces::{IMediator, INotification, IObserver, IView};
use crate::patterns::Observer;

static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

/// A PureMVC MultiCore [`IView`] implementation.
///
/// In PureMVC, [`IView`] implementors assume these responsibilities:
///
/// - Maintain a cache of [`IMediator`] instances.
/// - Provide methods for registering, retrieving, and removing [`IMediator`]s.
/// - Managing the [`IObserver`] lists for each [`INotification`].
/// - Providing a method for attaching [`IObserver`]s to an [`INotification`]'s [`IObserver`] list.
/// - Providing a method for broadcasting an [`INotification`] to each of the [`IObserver`]s in a list.
/// - Notifying the [`IObserver`]s of a given [`INotification`] when it broadcast.
///
/// See [`crate::interfaces::IMediator`], [`crate::interfaces::IObserver`], [`crate::interfaces::INotification`]
pub struct View {
    /// The Multiton key for this Core
    key: String,
    /// Mapping of [`INotification`] names to [`IObserver`] lists
    observer_map: RwLock<HashMap<String, Vec<Arc<dyn IObserver>>>>,
    /// Mapping of [`IMediator`] names to [`IMediator`] instances
    mediator_map: RwLock<HashMap<String, Arc<RwLock<dyn IMediator>>>>
}

impl View {
    /// Constructor.
    ///
    /// This [`IView`] implementation is a Multiton, so you should not call the constructor directly,
    /// but instead call the static [`View::get_instance`] method.
    ///
    /// # Panics
    /// if an instance for this Multiton key has already been constructed.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            observer_map: RwLock::new(HashMap::new()),
            mediator_map: RwLock::new(HashMap::new()),
        }
    }

    /// [`IView`] Multiton Factory method.
    ///
    /// Returns the [`IView`] Multiton instance for the specified key.
    pub fn get_instance<T: IView>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IView> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_view();
                Arc::new(instance)
            })
            .clone()
    }

    /// Remove an [`IView`] Multiton instance.
    ///
    /// # Arguments
    /// * `key` - The Multiton key of the [`IView`] instance to remove.
    pub fn remove_view(key: &str) {
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IView for View {
    /// Initialize the Multiton [`IView`] instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the Multiton
    /// instance in your subclass without overriding the constructor.
    fn initialize_view(&self){

    }

    /// Register an [`IObserver`] to be notified of [`INotification`]s with a given name.
    ///
    /// # Arguments
    /// * `notification_name` - The name of the [`INotification`] to notify this [`IObserver`] of.
    /// * `observer` - The [`IObserver`] to register.
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) {
        self.observer_map.write().ok()
            .map(|mut map| {
                map.entry(notification_name.into())
                    .or_default()
                    .push(observer);
            });
    }

    /// Remove an [`IObserver`] from the list for a given [`INotification`] name.
    ///
    /// # Arguments
    /// * `notification_name` - Which [`IObserver`] list to remove from.
    /// * `context` - Remove [`IObserver`]s with this object as the notify context.
    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) {
        self.observer_map.write().ok()
            .and_then(|mut map| map.get_mut(notification_name).cloned())
            .map(|mut observers| {
                observers.retain(|observer| !observer.compare_notify_context(&context));

                if observers.is_empty() {
                    self.observer_map.write().ok()
                        .map(|mut map| map.remove(notification_name));
                }
            });
    }

    /// Notify the [`IObserver`]s for a particular [`INotification`].
    ///
    /// All previously attached [`IObserver`]s for this [`INotification`]'s list are notified and are
    /// passed a reference to the [`INotification`] in the order in which they were registered.
    ///
    /// # Arguments
    /// * `notification` - The [`INotification`] to notify [`IObserver`]s of.
    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.observer_map.read().ok()
            .and_then(|map| map.get(notification.name()).cloned())
            .map(|observers| {
                observers.iter().for_each(|observer| {
                    observer.notify_observer(notification);
                });
            });
    }

    /// Register an [`IMediator`] instance with the [`IView`].
    ///
    /// Registers the [`IMediator`] so that it can be retrieved by name, and interrogates the
    /// [`IMediator`] for its [`INotification`] interests.
    ///
    /// If the [`IMediator`] returns a list of [`INotification`] names to be notified about, an
    /// [`crate::patterns::Observer`] is created encapsulating the [`IMediator`] instance's
    /// `handle_notification` method and registering it as an [`IObserver`] for all
    /// [`INotification`]s the [`IMediator`] is interested in.
    ///
    /// # Arguments
    /// * `mediator` - A reference to the [`IMediator`] instance.
    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        {
            let name = mediator.read().unwrap().name().to_string();
            let mut map = self.mediator_map.write().unwrap();
            if map.contains_key(&name) { return }
            map.insert(name, Arc::clone(&mediator));
        }

        let notify = {
            let mediator = Arc::clone(&mediator);
            Arc::new(move |notification: &Arc<dyn INotification>| {
                mediator.write().unwrap().handle_notification(notification);
            })
        };

        for interest in mediator.read().unwrap().list_notification_interests() {
            let context = Arc::new(Arc::clone(&mediator));
            let observer = Observer::new(Some(notify.clone()), Some(context));
            self.register_observer(&interest, Arc::new(observer));
        }

        {
            let mut guard = mediator.write().unwrap();
            guard.initialize_notifier(&self.key);
            guard.on_register();
        }
    }

    /// Retrieve an [`IMediator`] from the [`IView`].
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the [`IMediator`] instance to retrieve.
    ///
    /// # Returns
    /// The [`IMediator`] instance previously registered in this core with the given `mediator_name`.
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.mediator_map.read().ok()
            .and_then(|map| map.get(mediator_name).cloned())
    }

    /// Check if an [`IMediator`] is registered with the [`IView`].
    ///
    /// # Arguments
    /// * `mediator_name` - The name of the [`IMediator`] you're looking for.
    ///
    /// # Returns
    /// `true` if an [`IMediator`] is registered in this core with the given `mediator_name`, otherwise `false`.
    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.read().ok()
            .map(|map| map.contains_key(mediator_name))
            .unwrap()
    }

    /// Remove an [`IMediator`] from the [`IView`].
    ///
    /// # Arguments
    /// * `mediator_name` - Name of the [`IMediator`] instance to be removed.
    ///
    /// # Returns
    /// The [`IMediator`] that was removed from this core's [`IView`].
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.mediator_map.write().ok()
            .and_then(|mut map| map.remove(mediator_name))
            .map(|mediator| {
                for interest in mediator.read().unwrap().list_notification_interests() {
                    self.remove_observer(&interest, Arc::new(Arc::clone(&mediator)));
                }
                mediator.write().unwrap().on_remove(); mediator })
    }
}
