use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{IProxy, Model, Proxy};

pub struct ModelTestProxy(Proxy);

impl ModelTestProxy {
    const NAME: &'static str = "TestProxy";
    const ON_REGISTER_CALLED: &'static str = "onRegister Called";
    const ON_REMOVE_CALLED: &'static str = "onRemove Called";

    fn new() -> Self {
        Self(Proxy::new(Some(Self::NAME), None))
    }
}

impl IProxy for ModelTestProxy {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn data(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.0.data()
    }

    fn on_register(&mut self) {
        self.0.set_data(Some(Box::new(Self::ON_REGISTER_CALLED)));
    }

    fn on_remove(&mut self) {
        self.0.set_data(Some(Box::new(Self::ON_REMOVE_CALLED)));
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

    if let Some(data_any) = retrieved_proxy.lock().unwrap().data() {
        if let Some(data) = data_any.downcast_ref::<Vec<String>>() {
            assert_eq!(data.len(), 3);
            assert_eq!(data[0], "red");
            assert_eq!(data[1], "green");
            assert_eq!(data[2], "blue");
        } else {
            panic!("Data exists but is not a Vec<String>");
        }
    } else {
        panic!("Proxy has no data");
    }
}

#[test]
fn test_on_register_and_on_remove() {
    let model = Model::get_instance("ModelTestKey4", |k| Arc::new(Model::new(k)));

    let proxy = Arc::new(Mutex::new(ModelTestProxy::new()));
    model.register_proxy(proxy.clone());

    let value = {
        proxy.lock().unwrap().data()
            .and_then(|d| d.downcast_ref::<&'static str>())
            .copied()
            .expect("Proxy data is missing or wrong type")
    };

    assert_eq!(value, ModelTestProxy::ON_REGISTER_CALLED);

    model.remove_proxy("TestProxy");

    let value2 = {
        proxy.lock().unwrap().data()
            .and_then(|d| d.downcast_ref::<&'static str>())
            .copied()
            .expect("Proxy data is missing or wrong type")
    };

    assert_eq!(value2, ModelTestProxy::ON_REMOVE_CALLED, "Expecting proxy.data() == ModelTestProxy::ON_REMOVE_CALLED");
}
