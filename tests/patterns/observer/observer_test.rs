use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
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
    let mut observer = Observer::new(None, None);

    let object = Rc::new(RefCell::new(Object::new()));
    observer.set_context(Some(object.clone()));

    observer.set_notify(Some(Rc::new({
        let context = object.clone();
        move |note: &mut dyn INotification| context.borrow_mut().execute(note)
    })));

    let mut note = Notification::new("TestNote", Some(Box::new(10.0)), None);
    observer.notify_observer(&mut note);

    assert_eq!(object.borrow().value, 10.0);
}

#[test]
fn test_observer_constructor() {
    let object = Rc::new(RefCell::new(Object::new()));

    let observer = Observer::new(Some(Rc::new({
        let context = object.clone();
        move |note: &mut dyn INotification| context.borrow_mut().execute(note)
    })), Some(object.clone()));

    let mut note = Notification::new("ObserverTestNote", Some(Box::new(5.0)), None);
    observer.notify_observer(&mut note);

    assert_eq!(object.borrow().value, 5.0);
}

#[test]
fn test_compare_notify_context() {
    //let object = Rc::new(RefCell::new(Object::new()));
    let object: Rc<dyn Any> = Rc::new(RefCell::new(Object::new()));
    let observer = Observer::new(
        Some(Rc::new({
            let context = object.clone();
            move |note: &mut dyn INotification| {
                context
                    .downcast_ref::<RefCell<Object>>()
                    .unwrap()
                    .borrow_mut()
                    .execute(note)
            }
        })),
        Some(object.clone()),
    );
    let neg_test_object: Rc<dyn Any> = Rc::new(RefCell::new(Object::new()));

    assert_eq!(observer.compare_notify_context(&object), true);
    assert_eq!(observer.compare_notify_context(&neg_test_object), false);
}