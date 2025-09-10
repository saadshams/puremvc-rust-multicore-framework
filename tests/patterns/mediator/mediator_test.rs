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
    let button: Arc<dyn Any + Send + Sync> = Arc::new(Button { label: "Click Me".to_string() });

    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&button)));

    mediator.component()
        .and_then(|weak| weak.upgrade())
        .and_then(|arc| arc.downcast::<Button>().ok())
        .map(|object| assert_eq!(object.label, "Click Me".to_string()) );
}
