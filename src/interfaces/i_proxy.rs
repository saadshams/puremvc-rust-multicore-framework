use std::any::Any;
use std::sync::Arc;
use crate::interfaces::INotifier;

pub trait IProxy: INotifier {
    fn name(&self) -> &str;

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        None
    }

    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        let _ = data;
    }
    
    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {

    }
}
