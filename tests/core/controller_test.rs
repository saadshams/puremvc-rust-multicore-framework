use std::sync::Arc;
use puremvc::{Controller};

#[test]
fn test_get_instance() {
    let controller = Controller::get_instance("ControllerTestKey1", |k| Box::new(Controller::new(k)));

    assert!(Arc::strong_count(&controller) > 0, "Expecting instance not null");

}