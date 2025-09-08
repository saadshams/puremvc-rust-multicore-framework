use std::sync::{Arc, Mutex};
use puremvc::{INotification, Notification, Observer, View};

struct Object {}

#[test]
fn test_get_instance() {
    let view = View::get_instance("ViewTestKey1", |k| Arc::new(View::new(k)));

    assert!(Arc::strong_count(&view) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_notify_observer() {
    let view = View::get_instance("ViewTestKey2", |k| Arc::new(View::new(k)));

    let view_test_var = Arc::new(Mutex::new(2));

    let notify = {
        let view_test_var = view_test_var.clone();
        move |notification: &Arc<Mutex<dyn INotification>>| {
            let note = notification.lock().unwrap();
            *view_test_var.lock().unwrap() = *note.body().unwrap().lock().unwrap().downcast_ref::<i32>().unwrap();
        }
    };

    let context = Arc::new(Mutex::new(Object{}));
    let observer = Observer::new(Some(Arc::new(notify)), Some(context));
    view.register_observer("ObserverTestNote", Arc::new(observer));

    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new("ObserverTestNote", Some(Arc::new(Mutex::new(10i32))), None)));
    view.notify_observers(&note);

    assert_eq!(*view_test_var.lock().unwrap(), 10);
}
