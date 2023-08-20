#![warn(clippy::pedantic)]

use log::{debug, info, warn, LevelFilter};

/// Outputs:
/// ```text
/// [2023-08-20T22:59:16Z TRACE log_instrument] one enter
/// [2023-08-20T22:59:16Z TRACE log_instrument] one exit
/// [2023-08-20T22:59:16Z INFO  five] None
/// [2023-08-20T22:59:16Z TRACE log_instrument] one enter
/// [2023-08-20T22:59:16Z DEBUG five] ["a", "b"]
/// [2023-08-20T22:59:16Z DEBUG five] 23
/// [2023-08-20T22:59:16Z TRACE log_instrument] one exit
/// [2023-08-20T22:59:16Z INFO  five] Some(["a", "b"])
/// ```
fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();
    let mut my_struct = MyStruct(None);
    info!("{:?}", my_struct.one());
    let mut my_struct = MyStruct(Some(vec![String::from("a"), String::from("b")]));
    info!("{:?}", my_struct.one());
}
struct MyStruct(Option<Vec<String>>);

impl MyStruct {
    #[log_instrument::instrument]
    fn one(&mut self) -> Option<&mut [String]> {
        const SOMETHING: u32 = 23;
        match &mut self.0 {
            Some(y) => {
                debug!("{y:?}");
                debug!("{SOMETHING}");
                Some(y)
            }
            _ => None,
        }
    }
}
