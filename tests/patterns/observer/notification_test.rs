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
    
    let body = note.body().cloned().unwrap();
    let mut guard = body.lock().unwrap();
    let vo = guard.downcast_mut::<i32>().unwrap();

    assert_eq!(*vo, 5);
}

#[test]
fn test_constructor() {
    let note = Notification::new("TestNote", Some(Arc::new(Mutex::new(5i32))), Some("TestNoteType"));
    
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");

    let body = note.body().cloned().unwrap();
    let mut guard = body.lock().unwrap();
    let vo = guard.downcast_mut::<i32>().unwrap();
    assert_eq!(*vo, 5);

    assert_eq!(note.get_type(), Some("TestNoteType"));
}

#[test]
fn test_to_string() {
    let note = Notification::new("TestNote", Some(Arc::new(Mutex::new(vec![1, 3, 5]))), Some("TestType"));
    
    let expected = "Notification Name: TestNote\nBody: Mutex { data: Any { .. }, poisoned: false, .. }\nType: TestType";
    
    assert_eq!(note.to_string(), expected);
}