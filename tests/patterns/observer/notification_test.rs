use std::sync::{Arc, RwLock};
use puremvc::interfaces::INotification;
use puremvc::patterns::Notification;

/// Tests setting and getting the name using Notification accessor methods.
///
/// Creates a `Notification` with name 'TestNote' and asserts that the `name`
/// method returns 'TestNote'.
#[test]
fn test_name_accessor() {
    // Create a notification with name 'TestNote'
    let note: &dyn INotification = &Notification::new("TestNote", None, None);
    // Assert that the notification's name is 'TestNote'
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");
}

/// Tests setting and getting the body using Notification accessor methods.
///
/// Creates a `Notification`, sets its body to 5 using `set_body`, and asserts
/// that the body is 5.
#[test]
fn test_body_accessor() {
    // Create a notification with no initial body
    let mut note = Notification::new("TestNote", None, None);
    // Set the notification's body to 5
    note.set_body(Some(Arc::new(RwLock::new(5))));

    // Retrieve and verify the body
    note.body()
        .and_then(|arc| arc.downcast_ref::<RwLock<i32>>())
        .and_then(|mutex| mutex.read().ok())
        .map(|val| {
            // Assert that the body is 5
            assert_eq!(*val, 5);
        });
}

/// Tests setting the name, body, and type using the Notification constructor.
///
/// Creates a `Notification` with name 'TestNote', body 5, and type 'TestNoteType',
/// and asserts that the name, body, and type are set correctly.
#[test]
fn test_constructor() {
    // Create a notification with name, body, and type
    let note = Notification::new("TestNote", Some(Arc::new(RwLock::new(5i32))), Some("TestNoteType"));

    // Assert that the notification's name is 'TestNote'
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");

    // Retrieve and verify the body
    note.body()
        .and_then(|arc| arc.downcast_ref::<RwLock<i32>>())
        .and_then(|mutex| mutex.read().ok())
        .map(|val| {
            // Assert that the body is 5
            assert_eq!(*val, 5);
        });

    // Assert that the notification's type is 'TestNoteType'
    assert_eq!(note.get_type(), Some("TestNoteType"));
}

/// Tests the toString method of the Notification.
///
/// Creates a `Notification` with name 'TestNote', a vector body [1, 3, 5], and
/// type 'TestType', and asserts that `to_string` produces the expected string.
#[test]
fn test_to_string() {
    // Create a notification with name, body, and type
    let note = Notification::new("TestNote", Some(Arc::new(RwLock::new(vec![1, 3, 5]))), Some("TestType"));

    // Define the expected string output
    let expected = "Notification Name: TestNote\nBody: Any { .. }\nType: TestType";

    // Assert that the to_string output matches the expected string
    assert_eq!(note.to_string(), expected);
}