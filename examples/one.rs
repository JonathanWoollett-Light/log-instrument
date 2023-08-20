use log::*;

/// Outputs:
/// ```text
/// [2023-08-20T19:29:32Z TRACE example] one enter
/// [2023-08-20T19:29:32Z DEBUG example] cmp: true
/// [2023-08-20T19:29:32Z TRACE example] one exit
/// [2023-08-20T19:29:32Z INFO  example] 4
/// [2023-08-20T19:29:32Z TRACE example] one enter
/// [2023-08-20T19:29:32Z DEBUG example] cmp: false
/// [2023-08-20T19:29:32Z TRACE example] one exit
/// [2023-08-20T19:29:32Z INFO  example] 6
/// [2023-08-20T19:29:32Z TRACE example] one enter
/// [2023-08-20T19:29:32Z DEBUG example] cmp: false
/// [2023-08-20T19:29:32Z TRACE example] one exit
/// [2023-08-20T19:29:32Z INFO  example] 7
/// ```
fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();
    info!("{}", one(2));
    info!("{}", one(3));
    info!("{}", one(4));
}
#[log_instrument::instrument]
fn one(x: u32) -> u32 {
    let cmp = x == 2;
    debug!("cmp: {cmp}");
    if cmp {
        return 4;
    }
    x + 3
}
