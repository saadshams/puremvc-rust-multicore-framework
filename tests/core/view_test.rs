use std::any::Any;
use std::sync::{Arc, Mutex, Weak};
use puremvc::{IMediator, INotification, INotifier, Mediator, Notification, Observer, View};

struct Object {}

struct ViewTestMediator {
    mediator: Mediator
}

impl ViewTestMediator {
    pub const NAME: &'static str = "ViewTestMediator";

    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {
            mediator: Mediator::new(Some(Self::NAME), component)
        }
    }
}

impl INotifier for ViewTestMediator {}

impl IMediator for ViewTestMediator {
    fn name(&self) -> &str {
        self.mediator.name()
    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.mediator.notifier_mut()
    }

    fn list_notification_interests(&self) -> Vec<String> {
        vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()]
    }
}

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

#[test]
fn test_register_and_retrieve_mediator() {
    let view = View::get_instance("ViewTestKey3", |k| Arc::new(View::new(k)));

    let component = Arc::downgrade(&Arc::new(Object{}));

    let view_test_mediator = ViewTestMediator::new(Some(component));
    view.register_mediator(Arc::new(Mutex::new(view_test_mediator)));
}

#[test]
fn test_has_mediator() {
    let view = View::get_instance("ViewTestKey4", |k| Arc::new(View::new(k)));

    let mediator = Mediator::new(Some("hasMediatorTest"), None);
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    assert_eq!(view.has_mediator("hasMediatorTest"), true, "Expecting view.has_mediator('hasMediatorTest') == true");

    view.remove_mediator("hasMediatorTest");

    assert_eq!(view.has_mediator("hasMediatorTest"), false, "Expecting view.has_mediator('hasMediatorTest') == false");
}

#[test]
fn test_register_and_remove_mediator() {
    let view = View::get_instance("ViewTestKey5", |k| Arc::new(View::new(k)));

    let component = Arc::downgrade(&Arc::new(Object{}));

    let mediator = Mediator::new(Some("testing"), Some(component));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let removed_mediator = view.remove_mediator("testing");

    assert!(removed_mediator.is_some());
    assert_eq!(removed_mediator.unwrap().lock().unwrap().name(), "testing", "Expecting removed_mediator.name() == 'testing'");
    assert!(view.retrieve_mediator("testing").is_none(), "Expecting view.retrieve_mediator('testing').is_none()");
}