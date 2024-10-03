mod octants;

use std::process::Command;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use nanoid::nanoid;
use qrcode::{Color, QrCode};

use octants::OCTANTS;

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
        for y in (0..self.height).step_by(4) {
            for x in (0..self.width).step_by(2) {
                let block = [
                    self.color(x, y),
                    self.color(x + 1, y),
                    self.color(x, y + 1),
                    self.color(x + 1, y + 1),
                    self.color(x, y + 2),
                    self.color(x + 1, y + 2),
                    self.color(x, y + 3),
                    self.color(x + 1, y + 3),
                ];
                print!("{}", Self::block_to_char(block));
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

    fn block_to_char(block: [Color; 8]) -> char {
        let mut binary_value = 0;
        for (i, cell) in block.into_iter().enumerate() {
            if cell == Color::Dark {
                binary_value |= 1 << i;
            }
        }
        OCTANTS[binary_value as usize]
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

fn main() {
    println!("{HELP_MESSAGE}");

    let service_name = format!("adbqr-{}", nanoid!(10));
    let password = nanoid!();

    let data = format!("WIFI:T:ADB;S:{service_name};P:{password};;");
    let qr_code = QrCode::new(data).unwrap();
    let qr_renderer = QrRenderer::new(qr_code);
    qr_renderer.render();

    let daemon = ServiceDaemon::new().unwrap();
    let receiver = daemon.browse(PAIRING_SERVICE).unwrap();

    loop {
        let service = receiver.recv().unwrap();
        if let ServiceEvent::ServiceResolved(info) = service {
            if info.get_fullname().starts_with(&service_name) {
                println!("Connecting to device...");

                let address = info.get_addresses_v4().into_iter().next().unwrap();
                let port = info.get_port();
                let adb_output = Command::new("adb")
                    .arg("pair")
                    .arg(format!("{}:{}", address, port))
                    .arg(&password)
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
    }
}
