mod devices;

use clap::Parser;
use devices::Rgb;

#[derive(Parser)]
/// Cross-platform LED Controller
struct Args {
    #[arg(long, default_value = "0xffffff")]
    /// 24-bit RGB value. E.g. 0xff00ff
    color: Rgb,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = devices::setup(args.color) {
        eprintln!("error setting up devices: {}", e);
    }
}
