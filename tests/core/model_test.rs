use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{IProxy, Model, Proxy};

struct ModelTestProxy {
    proxy: Proxy
}

impl ModelTestProxy {
    const NAME: &'static str = "TestProxy";
    const ON_REGISTER_CALLED: &'static str = "onRegister Called";
    const ON_REMOVE_CALLED: &'static str = "onRemove Called";

    fn new() -> Self {
        Self {
            proxy: Proxy::new(Some(Self::NAME), None)
        }
    }
}

impl IProxy for ModelTestProxy {
    fn name(&self) -> &str {
        self.proxy.name()
    }

    fn data(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.proxy.data()
    }

    fn data_mut(&mut self) -> Option<&mut (dyn Any + Send + Sync)> {
        self.proxy.data_mut()
    }

    fn set_data(&mut self, data: Option<Box<dyn Any + Send + Sync>>) {
        self.proxy.set_data(data);
    }

    fn on_register(&mut self) {
        self.set_data(Some(Box::new(Self::ON_REGISTER_CALLED)));
    }

    fn on_remove(&mut self) {
        self.set_data(Some(Box::new(Self::ON_REMOVE_CALLED)));
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

    let proxy_guard = proxy.lock().unwrap();
    let value = proxy_guard.data()
        .and_then(|d| d.downcast_ref::<&'static str>())
        .expect("Proxy data is missing or wrong type");

    assert_eq!(*value, ModelTestProxy::ON_REGISTER_CALLED);

    // let proxy_guard = proxy.lock().unwrap();
    // if let Some(data) = proxy_guard.data() {
    //     if let Some(data2) = data.downcast_ref::<&'static str>() {
    //         assert_eq!(*data2, ModelTestProxy::ON_REGISTER_CALLED); // is this correct?
    //     } else {
    //         panic!("Only tests that registering data");
    //     }
    // } else {
    //     panic!("Proxy has no data");
    // }

    // if let Some(data) = proxy_guard.data() {
    //     if let Some(value) = data.downcast_ref::<Box<String>>() {
    //         if let
    //         assert_eq!(value, ModelTestProxy::ON_REGISTER_CALLED);
    //     } else {
    //         panic!("OnRegister called without data");
    //     }
    // } else {
    //     panic!("Proxy has no data");
    // }

    model.remove_proxy("TestProxy");

    assert_eq!(proxy.lock().unwrap().data().unwrap().downcast_ref::<String>().unwrap(), ModelTestProxy::ON_REMOVE_CALLED,
               "Expecting proxy.data() == ModelTestProxy::ON_REMOVE_CALLED");
}
