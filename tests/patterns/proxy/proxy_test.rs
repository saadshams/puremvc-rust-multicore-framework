use std::sync::Arc;
use puremvc::interfaces::IProxy;
use puremvc::patterns::Proxy;

/// Tests getting the name using the Proxy class accessor method.
///
/// Creates a `Proxy` with no name to test the default name and a `Proxy` with
/// name 'TestProxy', asserting that the `name` method returns the expected
/// values and that data is absent in both cases.
#[test]
fn test_name_accessor() {
    // Create a proxy with no name to test the default
    let proxy: &dyn IProxy = &Proxy::new(None, None);
    // Assert that the default proxy name is Proxy::NAME
    assert_eq!(proxy.name(), Proxy::NAME);
    // Assert that no data is set
    assert!(proxy.data().is_none());

    // Create a proxy with name 'TestProxy'
    let proxy = Proxy::new(Some("TestProxy"), None);
    // Assert that the proxy name is 'TestProxy'
    assert_eq!(proxy.name(), "TestProxy");
    // Assert that no data is set
    assert!(proxy.data().is_none());
}

/// Tests setting and getting the data using Proxy class accessor methods.
///
/// Creates a `Proxy` with name 'colors', sets its data to a vector of
/// `["red", "green", "blue"]` using `set_data`, and asserts that the data has
/// a length of 3 and contains the expected values.
#[test]
fn test_data_accessors() {
    // Create a proxy with name 'colors'
    let mut proxy = Proxy::new(Some("colors"), None);
    // Set the proxy's data to a vector of colors
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    proxy.set_data(Some(Arc::new(colors)));

    // Retrieve and verify the proxy's data
    proxy.data()
        .and_then(|arc| arc.downcast_ref::<Vec<String>>())
        .map(|data| {
            // Assert that the data length is 3
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            // Assert that the first element is 'red'
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            // Assert that the second element is 'green'
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            // Assert that the third element is 'blue'
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        });
}

/// Tests initializing a Proxy using the constructor.
///
/// Creates a `Proxy` with name 'colors' and data vector
/// `["red", "green", "blue"]`, asserting that the name is 'colors' and the
/// data has a length of 3 with the expected values.
#[test]
fn test_constructor() {
    // Create a proxy with name 'colors' and a vector of colors
    let proxy = Proxy::new(Some("colors"), Some(Arc::new(vec!["red".to_string(), "green".to_string(), "blue".to_string()])));

    // Assert that the proxy name is 'colors'
    assert_eq!(proxy.name(), "colors", "Expecting Proxy.get_name() == 'colors'");
    // Retrieve and verify the proxy's data
    proxy.data()
        .and_then(|arc| arc.downcast_ref::<Vec<String>>())
        .map(|data| {
            // Assert that the data length is 3
            assert_eq!(data.len(), 3, "Expecting data.len() == 3");
            // Assert that the first element is 'red'
            assert_eq!(data[0], "red", "Expecting data[0] == 'red'");
            // Assert that the second element is 'green'
            assert_eq!(data[1], "green", "Expecting data[1] == 'green'");
            // Assert that the third element is 'blue'
            assert_eq!(data[2], "blue", "Expecting data[2] == 'blue'");
        });
}
