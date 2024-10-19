use std::{
    io::{self, Write},
    sync::mpsc,
    time::Duration,
};

use crossterm::{cursor, terminal, ExecutableCommand};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

const ENTER_DEVICE: &'static str = "Select a device by index: ";

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

pub fn select_device(service: &str) -> anyhow::Result<ServiceInfo> {
    let daemon = ServiceDaemon::new()?;
    let receiver = daemon.browse(service)?;

    // Make a separate thread to read from stdin, so we can print new devices while waiting for input
    let (stdin_tx, stdin_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();
    let _stdin_handle = std::thread::spawn(move || while control_rx.recv().unwrap() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stdin_tx.send(input.trim_end().to_string()).unwrap();
    });

    let mut devices = Vec::new();

    print_devices(&devices);
    control_tx.send(true).unwrap();

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
                    control_tx.send(false).unwrap();
                    return Ok(info.clone());
                }
            }
            println!("Invalid input.");
            print_devices(&devices);
            control_tx.send(true).unwrap();
        }
    }
}
