use std::any::Any;
use std::sync::Arc;
use crate::interfaces::{INotifier, IModel};

/// The trait definition for a PureMVC MultiCore `IProxy`.
///
/// In PureMVC, an `IProxy` implementor assumes these responsibilities:
///
/// - Implement a common method which returns the name of the `IProxy`.
/// - Provide methods for setting and getting a Data Object.
///
/// Additionally, `IProxy`s typically:
///
/// - Provide methods for manipulating the Data Object and referencing it by type.
/// - Generate `INotification`s when their Data Object changes.
/// - Expose their name as a constant string called `NAME`.
/// - Encapsulate interaction with local or remote services used to fetch and persist data.
///
/// See [`IModel`]
pub trait IProxy: INotifier {
    /// Get the `Proxy` instance's name.
    ///
    /// # Returns
    /// The name of the `Proxy` instance.
    fn name(&self) -> &str;

    /// Get the `Proxy`'s Data Object.
    ///
    /// # Returns
    /// The Data Object associated with the `Proxy`.
    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>>;

    /// Set the `Proxy`'s Data Object.
    ///
    /// # Arguments
    /// * `data` - The Data Object this `Proxy` will tend.
    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>);

    /// Called by the `IModel` when the `Proxy` is registered.
    fn on_register(&mut self) {

    }

    /// Called by the `IModel` when the `Proxy` is removed.
    fn on_remove(&mut self) {

    }

    /// Get the `Proxy` as a dynamic `Any` type.
    ///
    /// # Returns
    /// The `Proxy` instance as a mutable `Any` reference.
    fn as_any(&mut self) -> &mut dyn Any;
}
