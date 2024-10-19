use std::{
    io::{self, Write}, process::Command, sync::mpsc, time::Duration
};

use crossterm::{cursor, terminal, ExecutableCommand};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

const CONNECT_SERVICE: &'static str = "_adb-tls-connect._tcp.local.";
const ENTER_DEVICE: &'static str = "Enter a number to connect to that device: ";

fn print_devices(devices: &[ServiceInfo]) {
    println!("Available devices:");
    for (index, info) in devices.iter().enumerate() {
        let address = info.get_addresses_v4().into_iter().next().unwrap();
        println!(
            "{}. {} ({}:{})",
            index + 1,
            info.get_hostname().trim_end_matches(".local."),
            address,
            info.get_port()
        );
    }
    print!("{ENTER_DEVICE}");
    io::stdout().flush().unwrap();
}

pub fn run() -> anyhow::Result<()> {
    let daemon = ServiceDaemon::new().unwrap();
    let receiver = daemon.browse(CONNECT_SERVICE).unwrap();

    let (stdin_tx, stdin_rx) = mpsc::channel();
    let _stdin_handle = std::thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stdin_tx.send(input.trim_end().to_string()).unwrap();
    });

    let mut devices = Vec::new();

    print_devices(&devices);

    loop {
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                devices.push(info.clone());

                let address = info.get_addresses_v4().into_iter().next().unwrap();

                io::stdout()
                    .execute(terminal::Clear(terminal::ClearType::CurrentLine))?
                    .execute(cursor::MoveToColumn(0))?;

                println!(
                    "{}. {} ({}:{})",
                    devices.len(),
                    info.get_hostname().trim_end_matches(".local."),
                    address,
                    info.get_port()
                );

                print!("{ENTER_DEVICE}");
                io::stdout().flush().unwrap();
            }
            Ok(_) => {}
            Err(flume::RecvTimeoutError::Timeout) => {}
            Err(e) => return Err(e.into()),
        }

        if let Ok(input) = stdin_rx.try_recv() {
            if let Ok(index) = input.parse::<usize>() {
                if let Some(info) = devices.get(index.wrapping_sub(1)) {
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

                    break;
                }
            }
            println!("Invalid input.");
            print_devices(&devices);
        }
    }

    Ok(())
}
