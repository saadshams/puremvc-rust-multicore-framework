use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{INotification, IObserver, Notification, Observer};

struct Object {
    value: f64,
}

impl Object {
    fn new() -> Self {
        Self { value: 0.0 }
    }

    fn execute(&mut self, note: &mut dyn INotification) {
        if let Some(value) = note.body().and_then(|v| v.downcast_ref::<f64>()) {
            self.value = *value;
        }
    }
}

#[test]
fn test_observer_accessors() {
    let object = Arc::new(Mutex::new(Object::new()));

    let mut observer = Observer::new(None, None);
    observer.set_context(Some(Arc::new(Box::new(object.clone()))));

    observer.set_notify(Some(Arc::new({
        let context = object.clone();
        move |note: &mut dyn INotification| {
            context.lock().unwrap().execute(note);
        }
    })));

    let mut note = Notification::new("TestNote", Some(Box::new(10.0)), None);
    observer.notify_observer(&mut note);

    // Check the updated value
    assert_eq!(object.lock().unwrap().value, 10.0);
}

#[test]
fn test_observer_constructor() {
    let object = Arc::new(Mutex::new(Object::new()));

    let observer = Observer::new(
        Some(Arc::new({
            let context = object.clone();
            move |note: &mut dyn INotification| {
                context.lock().unwrap().execute(note);
            }
        })),
        Some(Arc::new(Box::new(object.clone()) as Box<dyn Any + Send + Sync>)),
    );

    let mut note = Notification::new("ObserverTestNote", Some(Box::new(5.0)), None);
    observer.notify_observer(&mut note);

    assert_eq!(object.lock().unwrap().value, 5.0);
}

#[test]
fn test_compare_notify_context() {
    let object: Arc<Box<dyn Any + Send + Sync>> = Arc::new(Box::new(Mutex::new(Object::new())));

    let observer = Observer::new(
        Some(Arc::new({
            let context = object.clone();
            move |note: &mut dyn INotification| {
                let obj_ref = context.as_ref().as_ref().downcast_ref::<Mutex<Object>>().unwrap();
                obj_ref.lock().unwrap().execute(note);
            }
        })),
        Some(object.clone()),
    );

    let neg_test_object: Arc<Box<dyn Any + Send + Sync>> = Arc::new(Box::new(Mutex::new(Object::new())));

    assert!(observer.compare_notify_context(&object));
    assert!(!observer.compare_notify_context(&neg_test_object));
}
