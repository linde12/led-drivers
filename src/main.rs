mod devices;

fn main() {
    if let Err(e) = devices::setup() {
        eprintln!("error setting up devices: {}", e);
    }
}
