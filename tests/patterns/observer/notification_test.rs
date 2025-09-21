use std::sync::{Arc, RwLock};
use puremvc::interfaces::INotification;
use puremvc::patterns::Notification;

#[test]
fn test_name_accessor() {
    let note: &dyn INotification = &Notification::new("TestNote", None, None);
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");
}

#[test]
fn test_body_accessor() {
    let mut note = Notification::new("TestNote", None, None);
    note.set_body(Some(Arc::new(RwLock::new(5))));

    note.body()
        .and_then(|arc| arc.downcast_ref::<RwLock<i32>>())
        .and_then(|mutex| mutex.read().ok())
        .map(|val| {
            assert_eq!(*val, 5);
        });
}

#[test]
fn test_constructor() {
    let note = Notification::new("TestNote", Some(Arc::new(RwLock::new(5i32))), Some("TestNoteType"));
    
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");

    note.body()
        .and_then(|arc| arc.downcast_ref::<RwLock<i32>>())
        .and_then(|mutex| mutex.read().ok())
        .map(|val| {
            assert_eq!(*val, 5);
        });

    assert_eq!(note.get_type(), Some("TestNoteType"));
}

#[test]
fn test_to_string() {
    let note = Notification::new("TestNote", Some(Arc::new(RwLock::new(vec![1, 3, 5]))), Some("TestType"));
    
    let expected = "Notification Name: TestNote\nBody: Any { .. }\nType: TestType";
    
    assert_eq!(note.to_string(), expected);
}