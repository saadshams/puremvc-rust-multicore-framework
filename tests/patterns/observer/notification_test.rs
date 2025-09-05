use std::any::Any;
use puremvc::{INotification, Notification};


#[test]
fn test_name_accessor() {
    let note = Notification::new("TestNote", None, None);
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");
}

#[test]
fn test_body_accessor() {
    let mut note = Notification::new("TestNote", None, None);
    note.set_body(Option::from(Box::new(5) as Box<dyn Any+ Send + Sync>));
    
    let body = note.body().and_then(|b| b.downcast_ref::<i32>());
    assert_eq!(body, Some(&5));
}

#[test]
fn test_constructor() {
    let note = Notification::new("TestNote", Some(Box::new(5_i32)), Some("TestNoteType"));
    
    assert_eq!(note.name(), "TestNote", "Expecting note.get_name() == 'TestNote'");
    assert_eq!(note.body().and_then(|b| b.downcast_ref::<i32>()), Some(&5));
    assert_eq!(note.get_type(), Some("TestNoteType"));
}

#[test]
fn test_to_string() {
    let note = Notification::new("TestNote", Some(Box::new(vec![1, 3, 5]) as Box<dyn Any+ Send + Sync>), Some("TestType"));
    
    let expected = "Notification Name: TestNote\nBody: Any { .. }\nType: TestType";
    
    assert_eq!(note.to_string(), expected);
}