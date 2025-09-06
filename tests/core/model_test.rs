use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{INotifier, IProxy, Model, Proxy};

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

impl INotifier for ModelTestProxy {}

impl IProxy for ModelTestProxy {
    fn name(&self) -> &str {
        self.proxy.name()
    }

    fn data(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.proxy.data()
    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.proxy.notifier_mut()
    }

    fn on_register(&mut self) {
        self.proxy.set_data(Some(Box::new(Self::ON_REGISTER_CALLED)));
    }

    fn on_remove(&mut self) {
        self.proxy.set_data(Some(Box::new(Self::ON_REMOVE_CALLED)));
    }
}

#[test]
fn test_get_instance() {
    let model = Model::get_instance("ModelTestKey1", |k| Arc::new(Model::new(k)));

    assert!(Arc::strong_count(&model) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_retrieve_proxy() {
    let model = Model::get_instance("ModelTestKey2", |k| Arc::new(Model::new(k)));

    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Box::new(colors)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    let retrieved_proxy = model.retrieve_proxy("colors")
        .expect("Expecting proxy not null");

    let data = {
        retrieved_proxy.lock().unwrap().data()
            .expect("proxy has no data")
            .downcast_ref::<Vec<String>>()
            .expect("Data exists but is not a Vec<String>")
            .clone()
    };

    assert_eq!(data, &["red", "green", "blue"]);
}

#[test]
fn test_register_and_remove_proxy() {
    let model = Model::get_instance("ModelTestKey3", |k| Arc::new(Model::new(k)));

    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Box::new(sizes)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    let removed_proxy = model.remove_proxy("sizes")
        .expect("Expecting proxy not null");

    assert_eq!(removed_proxy.lock().unwrap().name(), "sizes", "Expecting named sizes");

    assert!(model.retrieve_proxy("sizes").is_none(), "Expecting sizes is none");
}

#[test]
fn test_has_proxy() {
    let model = Model::get_instance("ModelTestKey4", |k| Arc::new(Model::new(k)));

    let aces = vec!["clubs".to_string(), "spades".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("aces"), Some(Box::new(aces)));
    model.register_proxy(Arc::new(Mutex::new(proxy)));

    assert!(model.has_proxy("aces"), "Expecting model.has_proxy('aces') == true");

    model.remove_proxy("aces").expect("Expecting remove proxy aces");

    assert!(!model.has_proxy("aces"), "Expecting model.has_proxy('aces') == false");
}

#[test]
fn test_on_register_and_on_remove() {
    let model = Model::get_instance("ModelTestKey5", |k| Arc::new(Model::new(k)));

    let proxy = Arc::new(Mutex::new(ModelTestProxy::new()));
    model.register_proxy(proxy.clone());

    let value = {
        proxy.lock().unwrap().data()
            .and_then(|d| d.downcast_ref::<&'static str>())
            .copied()
            .expect("proxy data is missing or wrong type")
    };

    assert_eq!(value, ModelTestProxy::ON_REGISTER_CALLED);

    model.remove_proxy(ModelTestProxy::NAME);

    let value2 = {
        proxy.lock().unwrap().data()
            .and_then(|d| d.downcast_ref::<&'static str>())
            .copied()
            .expect("proxy data is missing or wrong type")
    };

    assert_eq!(value2, ModelTestProxy::ON_REMOVE_CALLED, "Expecting Proxy.data() == ModelTestProxy::ON_REMOVE_CALLED");
}
