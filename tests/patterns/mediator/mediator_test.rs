use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{IMediator, Mediator};

struct Button {
    label: String,
}

#[test]
fn test_name_accessor() {
    let mediator = Mediator::new(None, None);

    assert_eq!(mediator.name(), Mediator::NAME, "Expecting mediator.name() == Mediator::NAME");
}

#[test]
fn test_view_accessor() {
    let button = Button { label: "Click Me".to_string() };

    let component: Arc<dyn Any + Send + Sync> = Arc::new(button);
    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&component)));

    mediator.component()
        .and_then(|weak| weak.upgrade())
        .and_then(|arc| arc.downcast::<Button>().ok())
        .map(|object| {
            assert_eq!(object.label, "Click Me".to_string())
        })
        .expect("Downcasting failed");
}

#[test]
fn test_change_button_label() {
    let button = Button { label: "Click Me".to_string() };

    let component: Arc<dyn Any + Send + Sync> = Arc::new(Mutex::new(button));
    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&component)));

    mediator.component()
        .and_then(|weak| weak.upgrade())
        .and_then(|arc| arc.downcast::<Mutex<Button>>().ok())
        .map(|object| {
            object.lock().unwrap().label = "Button Changed".to_string();
        })
        .expect("Failed to change button label");

    // assert for the label changes
    assert_eq!(component.downcast::<Mutex<Button>>().ok().unwrap().lock().unwrap().label, "Button Changed".to_string());
}
