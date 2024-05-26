// constants
pub const PORT: &str = "8080";

pub type Job = Box<dyn FnOnce() + Send + 'static>;
