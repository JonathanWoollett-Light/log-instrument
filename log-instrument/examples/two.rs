use log::*;

/// Outputs:
/// ```text
/// [2023-08-20T22:21:51Z TRACE log_instrument] one enter
/// [2023-08-20T22:21:51Z TRACE log_instrument] one exit
/// [2023-08-20T22:21:51Z INFO  two] None
/// [2023-08-20T22:21:51Z TRACE log_instrument] one enter
/// [2023-08-20T22:21:51Z DEBUG two] ["a", "b"]
/// [2023-08-20T22:21:51Z TRACE log_instrument] one exit
/// [2023-08-20T22:21:51Z INFO  two] Some(["a", "b"]
/// ```
fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();
    info!("{:?}", one(&None));
    info!(
        "{:?}",
        one(&Some(vec![String::from("a"), String::from("b")]))
    );
}
#[log_instrument::instrument]
fn one(x: &Option<Vec<String>>) -> Option<&[String]> {
    match x {
        Some(y) => {
            debug!("{y:?}");
            Some(y)
        }
        _ => None,
    }
}
