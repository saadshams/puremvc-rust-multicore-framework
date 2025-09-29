use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, IFacade, INotification, INotifier, IController};
use crate::patterns::SimpleCommand;

/// A base `ICommand` implementation that synchronously executes other `ICommand`s.
///
/// A `MacroCommand` maintains a list of `ICommand` factories called SubCommands.
///
/// When `execute` is called, the `MacroCommand` instantiates and calls `execute` on each of its
/// SubCommands in turn. Each SubCommand will be passed a reference to the original
/// `INotification`.
///
/// Unlike `SimpleCommand`, your implementation should not override `execute`, but instead,
/// should override the `initialize_macro_command` method, calling `add_sub_command` once for
/// each SubCommand to be executed.
///
/// See [`ICommand`], [`IController`], [`INotification`], [`SimpleCommand`], [`INotifier`]
pub struct MacroCommand {
    /// The underlying `SimpleCommand` instance used for `INotifier` implementation.
    command: SimpleCommand,
    /// The list of SubCommand factories.
    sub_commands: Vec<Box<dyn Fn() -> Box<dyn ICommand + Send + Sync> + Send + Sync>>,
}

impl MacroCommand {
    /// Construct a new `MacroCommand`.
    ///
    /// You should not need to define a custom constructor; instead, override the
    /// `initialize_macro_command` method to initialize the SubCommand list.
    pub fn new() -> Self {
        Self {
            command: SimpleCommand::new(),
            sub_commands: Vec::new()
        }
    }

    /// Initialize the `MacroCommand`.
    ///
    /// In your implementation, override this method to initialize the `MacroCommand`'s SubCommand
    /// list with `ICommand` factories by calling `add_sub_command`.
    ///
    /// Note that SubCommands may be any `ICommand` implementor; `MacroCommand`s or
    /// `SimpleCommand`s are both acceptable.
    pub fn initialize_macro_command(&mut self) {

    }

    /// Add a SubCommand.
    ///
    /// The SubCommand will be called in First In/First Out (FIFO) order.
    ///
    /// # Arguments
    /// * `factory` - A function that constructs an instance of a `Command`.
    pub fn add_sub_command<T: ICommand + Send + Sync>(&mut self, factory: fn() -> T) {
        self.sub_commands.push(Box::new(move || Box::new(factory())));
    }
}

impl ICommand for MacroCommand {
    /// Execute this `MacroCommand`'s SubCommands.
    ///
    /// The SubCommands will be called in First In/First Out (FIFO) order.
    ///
    /// # Arguments
    /// * `notification` - The `Notification` object to be passed to each SubCommand.
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        for factory in self.sub_commands.drain(..) {
            let mut command = factory();
            command.initialize_notifier(self.command.key());
            command.execute(&notification);
        }
    }
}

impl INotifier for MacroCommand {
    /// Get the Multiton key for this `MacroCommand`.
    ///
    /// # Returns
    /// The Multiton key of the `MacroCommand`.
    fn key(&self) -> &str {
        self.command.key()
    }

    /// Get the `IFacade` instance associated with this `MacroCommand`.
    ///
    /// # Returns
    /// The `IFacade` instance.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    /// Initialize this `MacroCommand` instance.
    ///
    /// This is how a `MacroCommand` gets its Multiton key. Calls to `send_notification` or
    /// access to the `IFacade` will fail until after this method has been called.
    ///
    /// # Arguments
    /// * `key` - The Multiton key for this `MacroCommand`.
    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    /// Send a `Notification`.
    ///
    /// Convenience method to prevent having to construct new `Notification` instances in
    /// implementation code.
    ///
    /// # Arguments
    /// * `name` - The name of the `Notification` to send.
    /// * `body` - The body of the `Notification` (optional).
    /// * `type_` - The type of the `Notification` (optional).
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}
