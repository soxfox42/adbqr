use std::{io::{self, Write}, process::Command};

use crate::{select::select_device, PAIRING_SERVICE};

const HELP_MESSAGE: &'static str = "\
\x1B[1mPair with pairing code\x1B[0m

Make sure your Android device is on the same network as your computer.
Then, on your Android device:
1. Open \x1B[1mDeveloper options\x1B[0m.
2. Open \x1B[1mWireless debugging\x1B[0m, and enable it if necessary.
3. Select \x1B[1mPair device with pairing code\x1B[0m.
4. Complete these steps:
";

pub fn run() -> anyhow::Result<()> {
    println!("{HELP_MESSAGE}");

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