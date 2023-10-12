pub use log_instrument_macros::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Mutex;
use std::sync::OnceLock;

type InnerPath = Mutex<HashMap<std::thread::ThreadId, Vec<&'static str>>>;
static PATH: OnceLock<InnerPath> = OnceLock::new();
fn init_path() -> InnerPath {
    Mutex::new(HashMap::new())
}

pub struct __Instrument;

impl __Instrument {
    pub fn new(s: &'static str) -> __Instrument {
        // Get log
        let mut guard = PATH.get_or_init(init_path).lock().unwrap();
        let id = std::thread::current().id();
        let prefix = if let Some(spans) = guard.get_mut(&id) {
            let out = spans.iter().fold(String::new(), |mut s, x| {
                let _ = write!(s, "::{x}");
                s
            });
            spans.push(s);
            out
        } else {
            guard.insert(id, vec![s]);
            String::new()
        };

        // Write log
        log::trace!("{id:?}{prefix}>>{s}");

        // Return exit struct
        __Instrument
    }
}
impl std::ops::Drop for __Instrument {
    fn drop(&mut self) {
        // Get log
        let mut guard = PATH.get_or_init(init_path).lock().unwrap();
        let id = std::thread::current().id();
        let spans = guard.get_mut(&id).unwrap();
        let s = spans.pop().unwrap();
        let out = spans.iter().fold(String::new(), |mut s, x| {
            let _ = write!(s, "::{x}");
            s
        });
        log::trace!("{id:?}{out}<<{s}");
    }
}
