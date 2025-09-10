use std::sync::{Arc, Mutex};
use puremvc::{INotification, Notification};

#[test]
fn test_name_accessor() {
    let note = Notification::new("TestNote", None, None);
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");
}

#[test]
fn test_body_accessor() {
    let mut note = Notification::new("TestNote", None, None);
    note.set_body(Some(Arc::new(Mutex::new(5))));

    if let Some(body) = note.body() {
        let vo = body.downcast_ref::<Mutex<i32>>().unwrap().lock().unwrap();
        assert_eq!(*vo, 5);
    }
}

#[test]
fn test_constructor() {
    let note = Notification::new("TestNote", Some(Arc::new(Mutex::new(5i32))), Some("TestNoteType"));
    
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");

    if let Some(body) = note.body() {
        let vo = body.downcast_ref::<Mutex<i32>>().unwrap().lock().unwrap();

        assert_eq!(*vo, 5);
        assert_eq!(note.get_type(), Some("TestNoteType"));
    }
}

#[test]
fn test_to_string() {
    let note = Notification::new("TestNote", Some(Arc::new(Mutex::new(vec![1, 3, 5]))), Some("TestType"));
    
    let expected = "Notification Name: TestNote\nBody: Any { .. }\nType: TestType";
    
    assert_eq!(note.to_string(), expected);
}