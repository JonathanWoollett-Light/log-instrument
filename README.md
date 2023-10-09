# log-instrument

[![Crates.io](https://img.shields.io/crates/v/log-instrument)](https://crates.io/crates/log-instrument)
[![docs](https://img.shields.io/crates/v/log-instrument?color=yellow&label=docs)](https://docs.rs/log-instrument)
[![codecov](https://codecov.io/gh/JonathanWoollett-Light/log-instrument/branch/master/graph/badge.svg?token=II1xtnbCDX)](https://codecov.io/gh/JonathanWoollett-Light/log-instrument)

Offers an attribute procedural macro that adds [`log::trace!`](https://docs.rs/log/latest/log/macro.trace.html) events at the start and end of attributed functions.

### Example

```rust
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
```

Outputs:

```
[2023-08-30T13:58:20Z TRACE log_instrument] one enter
[2023-08-30T13:58:20Z DEBUG one] cmp: true
[2023-08-30T13:58:20Z TRACE log_instrument] one exit
[2023-08-30T13:58:20Z INFO  one] 4
[2023-08-30T13:58:20Z TRACE log_instrument] one enter
[2023-08-30T13:58:20Z DEBUG one] cmp: false
[2023-08-30T13:58:20Z TRACE log_instrument] one exit
[2023-08-30T13:58:20Z INFO  one] 6
[2023-08-30T13:58:20Z TRACE log_instrument] one enter
[2023-08-30T13:58:20Z DEBUG one] cmp: false
[2023-08-30T13:58:20Z TRACE log_instrument] one exit
[2023-08-30T13:58:20Z INFO  one] 7
```
