
use std::sync::Mutex;

lazy_static! {
    pub static ref MUSASHI_CORE_LOCK: Mutex<bool> = Mutex::new(true);
}
