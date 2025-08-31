use std::any::Any;
use std::sync::Arc;
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
    let button: Arc<dyn Any> = Arc::new(Button { label: "Click Me".to_string() });

    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&button)));

    assert!(mediator.component().is_some());
    if let Some(component) = mediator.component() {
        assert!(Arc::ptr_eq(&button, &component));

        if let Some(button_ref) = component.downcast_ref::<Button>() {
            assert_eq!(button_ref.label, "Click Me".to_string());
        }
    }

}
