use std::process::Command;

use crate::{select::select_device, CONNECT_SERVICE};

pub fn run() -> anyhow::Result<()> {
    let info = select_device(CONNECT_SERVICE)?;

    let address = info.get_addresses_v4().into_iter().next().unwrap();
    let port = info.get_port();

    println!("Connecting...");

    let adb_output = Command::new("adb")
        .arg("connect")
        .arg(format!("{}:{}", address, port))
        .output()
        .unwrap();

    if adb_output.status.success() {
        println!("Connected!");
    } else {
        println!("Connection failed.");
    }

    Ok(())
}
