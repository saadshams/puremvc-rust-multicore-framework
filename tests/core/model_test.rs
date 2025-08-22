use std::sync::Arc;
use puremvc::{Model, Proxy};

#[test]
fn test_get_instance() {
    let model = Model::get_instance("ModelTestKey1".to_string(), |k| Box::new(Model::new(k)));

    assert!(Arc::strong_count(&model) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_retrieve_proxy() {
    let model = Model::get_instance("ModelTestKey2".to_string(), |k| Box::new(Model::new(k)));

    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors".to_string()), Some(Box::new(colors)));
    model.register_proxy(Arc::new(proxy));

    let retrieved_proxy = model.retrieve_proxy("colors")
        .expect("Expecting proxy not null");

    if let Some(data_any) = retrieved_proxy.data() {
        if let Some(retrieved_colors) = data_any.downcast_ref::<Vec<String>>() {
            // Safe access: retrieved_colors is &Vec<String>
            assert_eq!(retrieved_colors.len(), 3);
            assert_eq!(retrieved_colors[0], "red");
            assert_eq!(retrieved_colors[1], "green");
            assert_eq!(retrieved_colors[2], "blue");
        } else {
            panic!("Data exists but is not a Vec<String>");
        }
    } else {
        panic!("Proxy has no data");
    }
}