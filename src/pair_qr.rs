use std::process::Command;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use nanoid::nanoid;
use qrcode::{Color, EcLevel, QrCode};

struct QrRenderer {
    colors: Vec<Color>,
    width: usize,
    height: usize,
}

impl QrRenderer {
    fn new(qr_code: QrCode) -> Self {
        let width = qr_code.width();
        let colors = qr_code.into_colors();
        let height = colors.len() / width;
        Self {
            colors,
            width,
            height,
        }
    }

    fn render(&self) {
        for y in (0..self.height).step_by(2) {
            for x in 0..self.width {
                let block = match (self.color(x, y), self.color(x, y + 1)) {
                    (Color::Light, Color::Light) => ' ',
                    (Color::Light, Color::Dark) => '▄',
                    (Color::Dark, Color::Light) => '▀',
                    (Color::Dark, Color::Dark) => '█',
                };
                print!("{}", block);
            }
            println!();
        }
    }

    fn color(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return Color::Light;
        }
        self.colors[y * self.width + x]
    }
}

const HELP_MESSAGE: &'static str = "\
\x1B[1mPair with QR code\x1B[0m

Make sure your Android device is on the same network as your computer.
Then, on your Android device:
1. Open \x1B[1mDeveloper options\x1B[0m.
2. Open \x1B[1mWireless debugging\x1B[0m, and enable it if necessary.
3. Select \x1B[1mPair with QR code\x1B[0m.
4. Scan the following QR code:
";

const PAIRING_SERVICE: &'static str = "_adb-tls-pairing._tcp.local.";

pub fn run() {
    println!("{HELP_MESSAGE}");

    let service_name = format!("adbqr-{}", nanoid!(4));
    let password = nanoid!(6);

    let data = format!("WIFI:T:ADB;S:{service_name};P:{password};;");
    let qr_code = QrCode::with_error_correction_level(data, EcLevel::L).unwrap();
    let qr_renderer = QrRenderer::new(qr_code);
    qr_renderer.render();

    let daemon = ServiceDaemon::new().unwrap();
    let receiver = daemon.browse(PAIRING_SERVICE).unwrap();

    loop {
        let service = receiver.recv().unwrap();
        if let ServiceEvent::ServiceResolved(info) = service {
            if info.get_fullname().starts_with(&service_name) {
                println!("Pairing...");

                let address = info.get_addresses_v4().into_iter().next().unwrap();
                let port = info.get_port();
                let adb_output = Command::new("adb")
                    .arg("pair")
                    .arg(format!("{}:{}", address, port))
                    .arg(&password)
                    .output()
                    .unwrap();

                if adb_output.status.success() {
                    println!("Paired!");
                } else {
                    println!("Pairing failed.");
                }

                break;
            }
        }
    }
}
