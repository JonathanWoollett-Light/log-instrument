use log::*;

/// Outputs:
/// ```text
/// [2023-08-20T22:22:43Z TRACE log_instrument] one enter
/// [2023-08-20T22:22:43Z TRACE log_instrument] one exit
/// [2023-08-20T22:22:43Z INFO  four] None
/// [2023-08-20T22:22:43Z TRACE log_instrument] one enter
/// [2023-08-20T22:22:43Z DEBUG four] ["a", "b"]
/// [2023-08-20T22:22:43Z TRACE log_instrument] one exit
/// [2023-08-20T22:22:43Z INFO  four] Some(["a", "b"])
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
        match &mut self.0 {
            Some(y) => {
                debug!("{y:?}");
                Some(y)
            }
            _ => None,
        }
    }
}
