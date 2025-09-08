use std::any::Any;
use std::sync::{Arc, Mutex, Weak};
use puremvc::{IMediator, INotification, INotifier, Mediator, Notification, Observer, View};

struct Object {
    on_register_called: bool,
    on_remove_called: bool
}

impl Default for Object {
    fn default() -> Self {
        Self {
            on_register_called: false,
            on_remove_called: false
        }
    }
}

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

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.mediator.notifier()
    }

    fn list_notification_interests(&self) -> Vec<String> {
        vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()]
    }
}

struct ViewTestMediator4 {
    mediator: Mediator
}

impl ViewTestMediator4 {
    pub const NAME: &'static str = "ViewTestMediator4";

    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {
            mediator: Mediator::new(Some(Self::NAME), component)
        }
    }
}

impl INotifier for ViewTestMediator4 {}

impl IMediator for ViewTestMediator4 {
    fn name(&self) -> &str {
        self.mediator.name()
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.mediator.notifier()
    }

    fn on_register(&mut self) {
        println!("component exists? {}", self.component().is_some());

        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| {
                object.lock().unwrap().on_register_called = true;
            });
    }

    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| {
                object.lock().unwrap().on_remove_called = true;
            });
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

    let context = Arc::new(Mutex::new(Object::default()));
    let observer = Observer::new(Some(Arc::new(notify)), Some(context));
    view.register_observer("ObserverTestNote", Arc::new(observer));

    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new("ObserverTestNote", Some(Arc::new(Mutex::new(10i32))), None)));
    view.notify_observers(&note);

    assert_eq!(*view_test_var.lock().unwrap(), 10);
}

#[test]
fn test_register_and_retrieve_mediator() {
    let view = View::get_instance("ViewTestKey3", |k| Arc::new(View::new(k)));

    let component = Arc::downgrade(&Arc::new(Object::default()));

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

    let component = Arc::downgrade(&Arc::new(Object::default()));

    let mediator = Mediator::new(Some("testing"), Some(component));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let removed_mediator = view.remove_mediator("testing");

    assert!(removed_mediator.is_some());
    assert_eq!(removed_mediator.unwrap().lock().unwrap().name(), "testing", "Expecting removed_mediator.name() == 'testing'");
    assert!(view.retrieve_mediator("testing").is_none(), "Expecting view.retrieve_mediator('testing').is_none()");
}

#[test]
fn test_on_register_and_on_remove() {
    let view = View::get_instance("ViewTestKey6", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));

    let mediator = ViewTestMediator4::new(Some(Arc::downgrade(&component).clone()));

    view.register_mediator(Arc::new(Mutex::new(mediator)));

    assert!(component.lock().unwrap().on_register_called, "Expecting component.on_register_called == true");

    view.remove_mediator(ViewTestMediator4::NAME);

    assert!(component.lock().unwrap().on_remove_called, "Expecting component.on_remove_called == true");
}
