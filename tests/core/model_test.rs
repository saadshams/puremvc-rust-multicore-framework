use std::sync::{Arc, Mutex};
use puremvc::{Model, Proxy};

#[test]
fn test_get_instance() {
    let model = Model::get_instance("ModelTestKey1", |k| Box::new(Model::new(k)));

    assert!(Arc::strong_count(&model) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_retrieve_proxy() {
    let model = Model::get_instance("ModelTestKey2", |k| Box::new(Model::new(k)));

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
