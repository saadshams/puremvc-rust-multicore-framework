use puremvc::{IProxy, Proxy};

#[test]
fn test_name_accessor() {
    let proxy = Proxy::new(None, None);
    assert_eq!(proxy.get_proxy_name(), Proxy::NAME);
    assert!(proxy.get_data().is_none());
    
    let proxy = Proxy::new(Some("TestProxy".to_string()), None);
    assert_eq!(proxy.get_proxy_name(), "TestProxy");
    assert!(proxy.get_data().is_none());
}

#[test]
fn test_data_accessors() {
    let mut proxy = Proxy::new(Some("colors".to_string()), None);
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    proxy.set_data(Some(Box::new(colors)));
    
    let data = proxy.get_data()
        .and_then(|any| any.downcast_ref::<Vec<String>>())
        .expect("Expected a Vec<String>");
    
    assert_eq!(data.len(), 3, "Expecting data.len() == 3");
    assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
    assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
    assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
}

#[test]
fn test_constructor() {
    let proxy = Proxy::new(Some("colors".to_string()), Some(Box::new(vec!["red".to_string(), "green".to_string(), "blue".to_string()])));
    let data = proxy.get_data()
        .and_then(|d| d.downcast_ref::<Vec<String>>())
        .expect("Expected a Vec<String>");
    
    assert_eq!(proxy.get_proxy_name(), "colors", "Expecting proxy.get_name() == 'colors'");
    assert_eq!(data.len(), 3, "Expecting data.len() == 3");
    assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
    assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
    assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
}
