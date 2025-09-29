use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::interfaces::{IProxy, IFacade};

/// The trait definition for a PureMVC MultiCore `IModel`.
///
/// In PureMVC, an `IModel` implementor provides access to `IProxy` objects by named lookup.
///
/// An `IModel` assumes these responsibilities:
///
/// - Maintain a cache of `IProxy` instances.
/// - Provide methods for registering, retrieving, and removing `IProxy` instances.
///
/// Your application must register `IProxy` instances with the `IModel`. Typically, you use an
/// `ICommand` to create and register `IProxy` instances once the `IFacade` has initialized the core
/// actors.
///
/// See [`IProxy`], [`IFacade`]
pub trait IModel: Any + Sync + Send {
    /// Initialize the `IModel` Multiton instance.
    ///
    /// Called automatically by the constructor, this is your opportunity to initialize the Multiton
    /// instance in your subclass without overriding the constructor.
    fn initialize_model(&self);

    /// Register a `Proxy` instance with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy` - An object reference to be held by the `IModel`.
    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>);

    /// Retrieve a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance to retrieve.
    ///
    /// # Returns
    /// The `Proxy` instance previously registered with the given `proxy_name`.
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>>;

    /// Check if a `Proxy` is registered with the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - The name of the `Proxy` instance you're looking for.
    ///
    /// # Returns
    /// `true` if a `Proxy` is currently registered with the given `proxy_name`, otherwise `false`.
    fn has_proxy(&self, proxy_name: &str) -> bool;

    /// Remove a `Proxy` instance from the `IModel`.
    ///
    /// # Arguments
    /// * `proxy_name` - Name of the `Proxy` instance to be removed.
    ///
    /// # Returns
    /// The `Proxy` that was removed from the `IModel`.
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>>;
}
