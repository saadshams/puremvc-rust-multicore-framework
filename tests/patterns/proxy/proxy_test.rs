use std::any::Any;
use std::sync::Arc;
use std::thread;
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

#[test]
fn test_spawning(){
    let shared_data: Arc<dyn Any + Send + Sync> = Arc::new(42u32);
    let mut proxy = Proxy::new(Some("example"), Some(shared_data.clone()));

    let data_for_thread = proxy.data().clone().unwrap();

    let handle = thread::spawn(move || {
        let value = data_for_thread.downcast_ref::<u32>().unwrap();
        println!("Thread received value: {}", value);
    });

    handle.join().unwrap();
}
