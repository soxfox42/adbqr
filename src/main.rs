mod connect;
mod select;
mod pair_code;
mod pair_qr;

use std::env;

pub const CONNECT_SERVICE: &'static str = "_adb-tls-connect._tcp.local.";
pub const PAIRING_SERVICE: &'static str = "_adb-tls-pairing._tcp.local.";

fn main() -> anyhow::Result<()> {
    let mode = env::args().nth(1).unwrap_or("pair".to_string());

    match mode.as_str() {
        "pair" => pair_qr::run(),
        "manual" => pair_code::run()?,
        "connect" => connect::run()?,
        _ => {}
    }

    Ok(())
}
