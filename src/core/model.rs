use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use crate::interfaces::{IModel, IProxy, IFacade};
static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

/// A PureMVC MultiCore `IModel` implementation.
///
/// In PureMVC, an `IModel` implementor provides access to `IProxy` objects by named lookup.
///
/// An `IModel` assumes these responsibilities:
///
/// - Maintain a cache of `IProxy` instances.
/// - Provide methods for registering, retrieving, and removing `IProxy` instances.
///
/// Your application must register `IProxy` instances with the `IModel`. Typically, you use an
/// `ICommand` to create and register `IProxy` instances once the `IFacade` has initialized the core actors.
///
/// See [`IProxy`], [`IFacade`]
pub struct Model {
    /// The Multiton Key for this Core
    key: String,
    /// Mapping of Proxy names to IProxy instances
    proxy_map: RwLock<HashMap<String, Arc<RwLock<dyn IProxy>>>>,
}

impl Model {
    /// Constructor.
    ///
    /// This `IModel` implementation is a Multiton, so you should not call the constructor directly,
    /// but instead call the static `Model::get_instance` method.
    ///
    /// Panics with `MultitonErrorModelExists` if an instance for this Multiton key has already been constructed.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            proxy_map: RwLock::new(HashMap::new())
        }
    }

    /// `IModel` Multiton Factory method.
    ///
    /// Returns the `IModel` Multiton instance for the specified key.
    pub fn get_instance<T: IModel>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IModel> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_model();
                Arc::new(instance)
            })
            .clone()
    }

    /// Remove an `IModel` instance.
    ///
    /// # Arguments
    /// * `key` - The multiton key of the `IModel` instance to remove
    pub fn remove_model(key: &str) {
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IModel for Model {
    /// Initialize the `IModel` Multiton instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the Multiton
    /// instance in your subclass without overriding the constructor.
    fn initialize_model(&self) {

    }

    /// Register a `Proxy` instance with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy` - An object reference to be held by the `IModel`.
    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>) {
        self.proxy_map.write().ok()
            .map(|mut map| {
                let mut guard = proxy.write().unwrap();
                map.insert(guard.name().into(), Arc::clone(&proxy));
                guard.initialize_notifier(&self.key);
                guard.on_register();
            });
    }

    /// Retrieve a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance to retrieve.
    ///
    /// # Returns
    /// The `Proxy` instance previously registered with the given `proxy_name`.
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.proxy_map.read().ok()
            .map(|map| map.get(proxy_name).cloned())
            .unwrap()
    }

    /// Check if a `Proxy` is registered with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance you're looking for.
    ///
    /// # Returns
    /// Returns `true` if a `Proxy` is currently registered with the given `proxy_name`, otherwise `false`.
    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.proxy_map.read().ok()
            .map(|map| map.contains_key(proxy_name))
            .unwrap()
    }

    /// Remove a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - Name of the `Proxy` instance to be removed.
    ///
    /// # Returns
    /// The `Proxy` that was removed from the `IModel`.
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.proxy_map.write().ok()
            .and_then(|mut map| map.remove(proxy_name))
            .map(|proxy| {
                proxy.write().unwrap().on_remove();
                proxy
            })
    }
}
