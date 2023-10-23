use log::*;

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
