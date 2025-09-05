use puremvc::{IProxy, Proxy};

#[test]
fn test_name_accessor() {
    let Proxy = Proxy::new(None, None);
    assert_eq!(Proxy.name(), Proxy::NAME);
    assert!(Proxy.data().is_none());
    
    let Proxy = Proxy::new(Some("TestProxy"), None);
    assert_eq!(Proxy.name(), "TestProxy");
    assert!(Proxy.data().is_none());
}

#[test]
fn test_data_accessors() {
    let mut Proxy = Proxy::new(Some("colors"), None);
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    Proxy.set_data(Some(Box::new(colors)));

    if let Some(data) = Proxy.data() {
        if let Some(data) = data.downcast_ref::<Vec<String>>() {
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        } else {
            panic!("data is None");
        }
    } else {
        panic!("Proxy.data() is None");
    }
}

#[test]
fn test_constructor() {
    let Proxy = Proxy::new(Some("colors"), Some(Box::new(vec!["red".to_string(), "green".to_string(), "blue".to_string()])));

    assert_eq!(Proxy.name(), "colors", "Expecting Proxy.get_name() == 'colors'");
    if let Some(data) = Proxy.data() {
        if let Some(data) = data.downcast_ref::<Vec<String>>() {
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        } else {
            panic!("data is None");
        }
    } else {
        panic!("Proxy.data() is None");
    }
}
