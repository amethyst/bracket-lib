mod input_handler;
pub use input_handler::*;
mod event_queue;
pub use event_queue::*;
use parking_lot::Mutex;

lazy_static! {
    pub static ref INPUT: Mutex<Input> = Mutex::new(Input::new());
}
