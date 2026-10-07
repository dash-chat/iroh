//! Prints the endpoint's advertised addresses whenever they change.
//!
//! Run it, then turn Wi-Fi off and back on. The address disappears as soon as
//! Wi-Fi goes down, but only reappears ~20-26 s after Wi-Fi is back up.
//!
//!     $ cargo run --example stale-direct-addrs
//!
//! or let `stale-direct-addrs.sh` toggle Wi-Fi for you.
use iroh::{Endpoint, RelayMode, endpoint::presets};
use n0_error::{Result, StdResultExt};
use n0_watcher::Watcher;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let endpoint = Endpoint::builder(presets::N0)
        .relay_mode(RelayMode::Disabled)
        .bind()
        .await?;
    let start = Instant::now();
    let mut addrs = endpoint.watch_addr();
    loop {
        let ips: Vec<_> = addrs.get().ip_addrs().map(|a| a.ip()).collect();
        println!(
            "{:>5.1} s  advertised {ips:?}",
            start.elapsed().as_secs_f64()
        );
        addrs.updated().await.std_context("endpoint closed")?;
    }
}
