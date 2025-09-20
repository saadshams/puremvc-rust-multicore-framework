use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::core::Model;
use puremvc::interfaces::{IFacade, INotifier, IProxy};
use puremvc::patterns::Proxy;

pub struct ModelTestProxy {
    proxy: Proxy,
}

impl ModelTestProxy {
    const NAME: &'static str = "TestProxy";
    const ON_REGISTER_CALLED: &'static str = "onRegister Called";
    const ON_REMOVE_CALLED: &'static str = "onRemove Called";

    fn new() -> Self {
        Self{proxy: Proxy::new(Some(Self::NAME), None)}
    }
}

impl INotifier for ModelTestProxy {
    fn key(&self) -> &str {
        self.proxy.key()
    }


    fn facade(&self) -> Arc<dyn IFacade> {
        self.proxy.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.proxy.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.proxy.send_notification(name, body, type_);
    }
}

impl IProxy for ModelTestProxy {
    fn name(&self) -> &str { self.proxy.name() }

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.proxy.data()
    }

    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.proxy.set_data(data);
    }

    fn on_register(&mut self) {
        self.proxy.set_data(Some(Arc::new(Self::ON_REGISTER_CALLED)));
    }

    fn on_remove(&mut self) {
        self.proxy.set_data(Some(Arc::new(Self::ON_REMOVE_CALLED)));
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[test]
fn test_get_instance() {
    let model = Model::get_instance("ModelTestKey1", |k| Model::new(k));

    assert!(Arc::strong_count(&model) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_retrieve_proxy() {
    let model = Model::get_instance("ModelTestKey2", |k| Model::new(k));

    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Arc::new(colors)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    model.retrieve_proxy("colors")
        .map(|proxy| {
            let guard = proxy.lock().expect("lock poisoned");
            let data = guard
                .data()
                .and_then(|d| d.downcast_ref::<Vec<String>>())
                .expect("invalid data type");
            assert_eq!(data, &["red", "green", "blue"]);
        })
        .expect("missing proxy");
}

#[test]
fn test_register_and_remove_proxy() {
    let model = Model::get_instance("ModelTestKey3", |k| Model::new(k));

    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Arc::new(sizes)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    model.remove_proxy("sizes")
        .map(|proxy| {
            let guard = proxy.lock().expect("lock poisoned");
            assert_eq!(guard.name(), "sizes", "Expecting named sizes");
        });

    assert!(model.retrieve_proxy("sizes").is_none(), "Expecting sizes is none");
}

#[test]
fn test_has_proxy() {
    let model = Model::get_instance("ModelTestKey4", |k| Model::new(k));

    let aces = vec!["clubs".to_string(), "spades".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("aces"), Some(Arc::new(aces)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    assert!(model.has_proxy("aces"), "Expecting model.has_proxy('aces') == true");

    model.remove_proxy("aces");

    assert!(!model.has_proxy("aces"), "Expecting model.has_proxy('aces') == false");
}

#[test]
fn test_on_register_and_on_remove() {
    let model = Model::get_instance("ModelTestKey5", |k| Model::new(k));

    let proxy = Arc::new(Mutex::new(ModelTestProxy::new()));
    model.register_proxy(proxy.clone());

    proxy.lock().unwrap()
        .data()
        .and_then(|arc| arc.downcast_ref::<&'static str>().copied())
        .map(|value| {
            assert_eq!(value, ModelTestProxy::ON_REGISTER_CALLED, "Expecting proxy.data() == ModelTestProxy::ON_REGISTER_CALLED");
        });
    
    model.remove_proxy(ModelTestProxy::NAME);

    proxy.lock().unwrap()
        .data()
        .and_then(|arc| arc.downcast_ref::<&'static str>().copied())
        .map(|value| {
            assert_eq!(value, ModelTestProxy::ON_REMOVE_CALLED, "Expecting proxy.data() == ModelTestProxy::ON_REMOVE_CALLED");
        });
}
