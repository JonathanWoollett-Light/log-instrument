pub use log_instrument_macros::*;

pub struct __Instrument<'a>(&'a str);
impl<'a> __Instrument<'a> {
    pub fn new(s: &'a str) -> __Instrument<'a> {
        log::trace!("{s} enter");
        __Instrument(s)
    }
}
impl<'a> std::ops::Drop for __Instrument<'a> {
    fn drop(&mut self) {
        log::trace!("{} exit", self.0);
    }
}
