use std::{io::{self, Write}, process::Command};

use crate::{select::select_device, PAIRING_SERVICE};

pub fn run() -> anyhow::Result<()> {
    let info = select_device(PAIRING_SERVICE)?;

    let address = info.get_addresses_v4().into_iter().next().unwrap();
    let port = info.get_port();

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    println!("Connecting...");

    let adb_output = Command::new("adb")
        .arg("pair")
        .arg(format!("{}:{}", address, port))
        .arg(password.trim_end())
        .output()
        .unwrap();

    if adb_output.status.success() {
        println!("Connected!");
    } else {
        println!("Connection failed.");
    }

    Ok(())
}