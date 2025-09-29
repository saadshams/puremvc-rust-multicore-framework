use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{INotification, IView, IObserver};

/// A base `INotification` implementation.
///
/// The Observer Pattern as implemented within PureMVC exists to support publish/subscribe
/// communication between actors.
///
/// `INotification`s are not meant to be a replacement for events, but rather an internal
/// communication mechanism that ensures PureMVC is portable regardless of what type of event
/// mechanism is supported (or not) on a given platform.
///
/// Generally, `IMediator` implementors place event listeners on their view components, and
/// `IProxy` implementors place event listeners on service components. Those events are then
/// handled in the usual way, and may lead to the broadcast of `INotification`s that trigger
/// `ICommand`s or notify `IMediator`s.
///
/// See [`IView`], [`IObserver`], [`Notification`]
pub struct Notification {
    /// The `Notification`'s name.
    name: String,
    /// The `Notification`'s body.
    body: Option<Arc<dyn Any + Send + Sync>>,
    /// The `Notification`'s type.
    type_: Option<String>,
}

impl Notification {
    /// Construct a new `Notification` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the `Notification`.
    /// * `body` - The body of the `Notification` (optional).
    /// * `type_` - The type of the `Notification` (optional).
    pub fn new(name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) -> Self {
        Self {
            name: name.into(),
            body,
            type_: type_.map(|t| t.into()),
        }
    }
}

impl INotification for Notification {
    /// Get the name of the `Notification`.
    ///
    /// # Returns
    /// The name of the `Notification`.
    fn name(&self) -> &str {
        &self.name
    }

    /// Get the body of the `Notification`.
    ///
    /// # Returns
    /// The body of the `Notification`.
    fn body(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.body.as_ref()
    }

    /// Set the body of the `Notification`.
    ///
    /// # Arguments
    /// * `body` - The body of the `Notification`.
    fn set_body(&mut self, body: Option<Arc<dyn Any + Send + Sync>>) {
        self.body = body;
    }

    /// Get the type of the `Notification`.
    ///
    /// # Returns
    /// The type of the `Notification`.
    fn get_type(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    /// Set the type of the `Notification`.
    ///
    /// # Arguments
    /// * `type_` - The type of the `Notification`.
    fn set_type(&mut self, type_: Option<String>) {
        self.type_ = type_;
    }

    /// Convert the `Notification` to a string representation.
    ///
    /// # Returns
    /// A string containing the name, body, and type of the `Notification`.
    fn to_string(&self) -> String {
        let name = &self.name;
        let body = match &self.body {
            Some(b) => format!("{:?}", b),
            None => "null".into()
        };
        let type_ = self.r#type_.as_deref().unwrap_or("null");
        format!("Notification Name: {}\nBody: {}\nType: {}", name, body, type_)
    }
}
