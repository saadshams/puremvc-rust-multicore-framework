use std::any::Any;
use puremvc::{IMediator, Mediator};

#[test]
fn test_name_accessor() {
    let mediator = Mediator::new(None, None);

    assert_eq!(mediator.name(), Mediator::NAME, "Expecting mediator.name() == Mediator::NAME");
}

#[test]
fn test_view_accessor() {
    let view = Box::new(()) as Box<dyn Any>;

    let mediator = Mediator::new(Some(Mediator::NAME), Some(view));
    assert!(mediator.component().is_some());
}
