use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::core::Model;
use puremvc::interfaces::{IFacade, INotifier, IProxy};
use puremvc::patterns::Proxy;

/// A Proxy subclass used by ModelTest.
pub struct ModelTestProxy {
    proxy: Proxy,
}

impl ModelTestProxy {
    const NAME: &'static str = "TestProxy";
    const ON_REGISTER_CALLED: &'static str = "onRegister Called";
    const ON_REMOVE_CALLED: &'static str = "onRemove Called";

    /// Constructor.
    fn new() -> Self {
        Self{proxy: Proxy::new(Some(Self::NAME), None)}
    }
}

impl INotifier for ModelTestProxy {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.proxy.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.proxy.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.proxy.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.proxy.send_notification(name, body, type_);
    }
}

impl IProxy for ModelTestProxy {
    /// Returns the name of the proxy.
    fn name(&self) -> &str { self.proxy.name() }

    /// Returns the data held by the proxy, if any.
    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.proxy.data()
    }

    /// Sets the data for the proxy.
    ///
    /// # Arguments
    /// * `data` - Optional data to be stored in the proxy
    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.proxy.set_data(data);
    }

    /// Called when the proxy is registered.
    fn on_register(&mut self) {
        self.proxy.set_data(Some(Arc::new(Self::ON_REGISTER_CALLED)));
    }

    /// Called when the proxy is removed.
    fn on_remove(&mut self) {
        self.proxy.set_data(Some(Arc::new(Self::ON_REMOVE_CALLED)));
    }

    /// Returns a mutable reference to the proxy as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Tests the Model Multiton Factory Method.
#[test]
fn test_get_instance() {
    let model = Model::get_instance("ModelTestKey1", |k| Model::new(k));

    assert!(Arc::strong_count(&model) > 0, "Expecting instance not null");
}

/// Tests the proxy registration and retrieval methods.
///
/// Tests `register_proxy` and `retrieve_proxy` in the same test.
/// These methods cannot currently be tested separately in any
/// meaningful way other than to show that the methods do not
/// throw exceptions when called.
#[test]
fn test_register_and_retrieve_proxy() {
    // Get a Multiton Model instance
    let model = Model::get_instance("ModelTestKey2", |k| Model::new(k));

    // Create and register a proxy with color data
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Arc::new(colors)));
    model.register_proxy(Arc::new(RwLock::new(proxy)));

    // Retrieve the proxy and verify its data
    model.retrieve_proxy("colors")
        .map(|proxy| {
            // Acquire a read lock on the proxy
            let guard = proxy.read().expect("lock poisoned");
            // Get and downcast the proxy data to Vec<String>
            let data = guard
                .data()
                .and_then(|d| d.downcast_ref::<Vec<String>>())
                .expect("invalid data type");
            // Assert that the data matches the expected colors
            assert_eq!(data, &["red", "green", "blue"]);
        })
        .expect("missing proxy");
}

/// Tests the proxy removal method.
#[test]
fn test_register_and_remove_proxy() {
    // Get a Multiton Model instance
    let model = Model::get_instance("ModelTestKey3", |k| Model::new(k));

    // Create and register a proxy with size data
    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Arc::new(sizes)));
    model.register_proxy(Arc::new(RwLock::new(proxy)));

    // Remove the proxy and verify its name
    model.remove_proxy("sizes")
        .map(|proxy| {
            // Acquire a read lock on the removed proxy
            let guard = proxy.read().expect("lock poisoned");
            // Assert that the removed proxy has the expected name
            assert_eq!(guard.name(), "sizes", "Expecting named sizes");
        });

    // Verify that the proxy is no longer retrievable
    assert!(model.retrieve_proxy("sizes").is_none(), "Expecting sizes is none");
}

/// Tests the `has_proxy` method.
#[test]
fn test_has_proxy() {
    // Get a Multiton Model instance
    let model = Model::get_instance("ModelTestKey4", |k| Model::new(k));

    // Create and register a proxy with card suit data
    let aces = vec!["clubs".to_string(), "spades".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("aces"), Some(Arc::new(aces)));
    model.register_proxy(Arc::new(RwLock::new(proxy)));

    // Assert that has_proxy returns true for the registered proxy
    assert!(model.has_proxy("aces"), "Expecting model.has_proxy('aces') == true");

    // Remove the proxy
    model.remove_proxy("aces");

    // Assert that has_proxy returns false after removal
    assert!(!model.has_proxy("aces"), "Expecting model.has_proxy('aces') == false");
}

/// Tests that the Model calls the `on_register` and `on_remove` methods.
#[test]
fn test_on_register_and_on_remove() {
    // Get a Multiton Model instance
    let model = Model::get_instance("ModelTestKey5", |k| Model::new(k));

    // Create and register a test proxy
    let proxy = Arc::new(RwLock::new(ModelTestProxy::new()));
    model.register_proxy(proxy.clone());

    // Verify that on_register was called by checking the proxy's data
    proxy.read().unwrap()
        .data()
        .and_then(|arc| arc.downcast_ref::<&'static str>().copied())
        .map(|value| {
            // Assert that the proxy data indicates on_register was called
            assert_eq!(value, ModelTestProxy::ON_REGISTER_CALLED, "Expecting proxy.data() == ModelTestProxy::ON_REGISTER_CALLED");
        });

    // Remove the proxy
    model.remove_proxy(ModelTestProxy::NAME);

    // Verify that on_remove was called by checking the proxy's data
    proxy.read().unwrap()
        .data()
        .and_then(|arc| arc.downcast_ref::<&'static str>().copied())
        .map(|value| {
            // Assert that the proxy data indicates on_remove was called
            assert_eq!(value, ModelTestProxy::ON_REMOVE_CALLED, "Expecting proxy.data() == ModelTestProxy::ON_REMOVE_CALLED");
        });
}
