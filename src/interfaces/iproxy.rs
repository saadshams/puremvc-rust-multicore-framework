use std::any::Any;
use std::sync::Arc;
use crate::INotifier;

pub trait IProxy: INotifier {
    fn name(&self) -> &str;

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        None
    }

    fn set_data(&mut self, _data: Option<Arc<dyn Any + Send + Sync>>) {

    }
    
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync>;

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {

    }
}
