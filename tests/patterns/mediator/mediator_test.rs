use std::any::Any;
use std::rc::Rc;
use puremvc::{IMediator, Mediator};

#[test]
fn test_name_accessor() {
    let mediator = Mediator::new(None, None);

    assert_eq!(mediator.name(), Mediator::NAME, "Expecting mediator.name() == Mediator::NAME");
}

#[test]
fn test_view_accessor() {
    let view: Rc<dyn Any> = Rc::new(());

    let mediator = Mediator::new(Some("MyMediator"), Some(Rc::clone(&view)));
    assert!(mediator.component().is_some());
}
