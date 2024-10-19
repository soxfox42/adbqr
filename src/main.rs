mod connect;
mod pair_code;
mod pair_qr;
mod select;

use std::{env, process};

pub const CONNECT_SERVICE: &'static str = "_adb-tls-connect._tcp.local.";
pub const PAIRING_SERVICE: &'static str = "_adb-tls-pairing._tcp.local.";

fn main() -> anyhow::Result<()> {
    let mode = env::args().nth(1).unwrap_or("pair".to_string());

    match mode.as_str() {
        "pair" => pair_qr::run(),
        "manual" => pair_code::run()?,
        "connect" => connect::run()?,
        "-h" | "--help" => {
            println!("Usage: adbqr [pair|manual|connect]");
        }
        unknown => {
            eprintln!("Unknown command: {}", unknown);
            process::exit(1);
        }
    }

    Ok(())
}
