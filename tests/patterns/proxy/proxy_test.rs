use std::sync::Arc;
use puremvc::{IProxy, Proxy};

#[test]
fn test_name_accessor() {
    let mut proxy = Proxy::new(None, None);
    assert_eq!(proxy.name(), Proxy::NAME);
    assert!(proxy.data().is_none());
    
    let mut proxy = Proxy::new(Some("TestProxy"), None);
    assert_eq!(proxy.name(), "TestProxy");
    assert!(proxy.data().is_none());
}

#[test]
fn test_data_accessors() {
    let mut proxy = Proxy::new(Some("colors"), None);
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    proxy.set_data(Some(Arc::new(colors)));

    if let Some(data) = proxy.data() {
        if let Some(data) = data.downcast_ref::<Vec<&str>>() {
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        }
    }
}

#[test]
fn test_constructor() {
    let mut proxy = Proxy::new(Some("colors"), Some(Arc::new(vec!["red".to_string(), "green".to_string(), "blue".to_string()])));

    assert_eq!(proxy.name(), "colors", "Expecting proxy.get_name() == 'colors'");
    if let Some(data) = proxy.data() {
        if let Some(data) = data.downcast_ref::<Vec<&str>>() {
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        }
    }
}
