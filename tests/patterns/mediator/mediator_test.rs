use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::IMediator;
use puremvc::patterns::Mediator;

/// A utility struct to simulate a button component for testing.
struct Button {
    label: String,
}

/// Tests the name accessor of the Mediator class.
///
/// Creates a `Mediator` with no specified name and asserts that the `name`
/// method returns `Mediator::NAME`.
#[test]
fn test_name_accessor() {
    // Create a mediator with no specified name
    let mediator: &dyn IMediator = &Mediator::new(None, None);

    // Assert that the mediator's name is the default Mediator::NAME
    assert_eq!(mediator.name(), Mediator::NAME, "Expecting mediator.name() == Mediator::NAME");
}

/// Tests the view component accessor of the Mediator class.
///
/// Creates a `Mediator` with a `Button` component and asserts that the
/// view component can be retrieved and downcast to verify its label.
#[test]
fn test_view_accessor() {
    // Create a button component with a label
    let button = Button { label: "Click Me".to_string() };

    // Create a mediator with the button as its view component
    let component: Arc<dyn Any + Send + Sync> = Arc::new(button);
    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&component)));

    // Retrieve and downcast the view component to verify its label
    mediator.component()
        .and_then(|weak| weak.upgrade())
        .and_then(|arc| arc.downcast::<Button>().ok())
        .map(|object| {
            // Assert that the button's label is as expected
            assert_eq!(object.label, "Click Me".to_string())
        })
        .expect("Downcasting failed");
}

/// Tests modifying a button's label via the Mediator's view component.
///
/// Creates a `Mediator` with a `Button` component, modifies the button’s
/// label through the mediator, and asserts that the label has changed to
/// "Button Changed".
#[test]
fn test_change_button_label() {
    // Create a button component with an initial label
    let button = Button { label: "Click Me".to_string() };

    // Create a mediator with the button as its view component
    let component: Arc<dyn Any + Send + Sync> = Arc::new(RwLock::new(button));
    let mediator = Mediator::new(Some("MyMediator"), Some(Arc::downgrade(&component)));

    // Modify the button's label through the mediator
    mediator.component()
        .and_then(|weak| weak.upgrade())
        .and_then(|arc| arc.downcast::<RwLock<Button>>().ok())
        .map(|object| {
            // Change the button's label
            object.write().unwrap().label = "Button Changed".to_string();
        })
        .expect("Failed to change button label");

    // Assert that the button's label has been updated
    assert_eq!(component.downcast::<RwLock<Button>>().ok().unwrap().read().unwrap().label, "Button Changed".to_string());
}
