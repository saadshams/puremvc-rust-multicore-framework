use std::any::Any;
use puremvc_rust_multicore_framework::interfaces::IMediator;
use puremvc_rust_multicore_framework::patterns::mediator::Mediator;

#[test]
fn test_name_accessor() {
    let mediator = Mediator::new(None, None);

    assert_eq!(mediator.get_mediator_name(), Mediator::NAME, "Expecting mediator.get_name() == Mediator::NAME");
}

#[test]
fn test_view_accessor() {
    let view = Box::new(()) as Box<dyn Any>;

    let mediator = Mediator::new(Some(Mediator::NAME.to_string()), Some(view));
    assert!(mediator.get_view_component().is_some());
}
