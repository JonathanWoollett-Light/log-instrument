use log::*;

/// Outputs:
/// ```text
/// [2023-08-20T22:22:25Z TRACE log_instrument] one enter
/// [2023-08-20T22:22:25Z TRACE log_instrument] one exit
/// [2023-08-20T22:22:25Z INFO  three] None
/// [2023-08-20T22:22:25Z TRACE log_instrument] one enter
/// [2023-08-20T22:22:25Z DEBUG three] ["a", "b"]
/// [2023-08-20T22:22:25Z TRACE log_instrument] one exit
/// [2023-08-20T22:22:25Z INFO  three] Some(["a", "b"])
/// ```
fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();
    info!("{:?}", one(&mut None));
    info!(
        "{:?}",
        one(&mut Some(vec![String::from("a"), String::from("b")]))
    );
}
#[log_instrument::instrument]
fn one(x: &mut Option<Vec<String>>) -> Option<&mut [String]> {
    match x {
        Some(y) => {
            debug!("{y:?}");
            Some(y)
        }
        _ => None,
    }
}
