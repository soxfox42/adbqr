mod connect;
mod pair_code;
mod pair_qr;

use std::env;

fn main() -> anyhow::Result<()> {
    let mode = env::args().nth(1).unwrap_or("pair".to_string());

    match mode.as_str() {
        "pair" => pair_qr::run(),
        "connect" => connect::run()?,
        _ => {}
    }

    Ok(())
}
