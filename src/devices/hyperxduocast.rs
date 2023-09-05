use std::time::Duration;

use hidapi::{HidApi, HidResult};

use super::Rgb;

/// HyperX DuoCast Controller
const VENDOR_ID: u16 = 0x03f0;
const PRODUCT_ID: u16 = 0x098c;

const TRANSACTION_START: [u8; 9] = [0x00, 0x04, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];

const TRANSACTION_END: [u8; 65] = {
    let mut buffer = [0u8; 65];
    buffer[0x01] = 0x08;
    buffer[0x3C] = 0x28;
    buffer[0x3D] = 0x01;
    buffer[0x3E] = 0x00;
    buffer[0x3F] = 0xAA;
    buffer[0x40] = 0x55;
    buffer
};

fn set_color_pkt(Rgb(r, g, b): Rgb) -> [u8; 9] {
    let mut buffer: [u8; 9] = [0u8; 9];
    buffer[0x01] = 0x81;
    buffer[0x02] = r;
    buffer[0x03] = g;
    buffer[0x04] = b;
    buffer[0x05] = 0x81;
    buffer[0x06] = r;
    buffer[0x07] = g;
    buffer[0x08] = b;
    buffer
}

pub fn setup(color: Rgb) -> HidResult<()> {
    let api = HidApi::new()?;

    for device_info in api.device_list() {
        if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
            // TODO: reverse-engineer out how to save to on-board memory if there is any...
            let device = api.open(VENDOR_ID, PRODUCT_ID)?;

            device.send_feature_report(&TRANSACTION_START)?;
            std::thread::sleep(Duration::from_millis(15));

            device.send_feature_report(&set_color_pkt(color))?;
            std::thread::sleep(Duration::from_millis(15));

            device.send_feature_report(&TRANSACTION_END)?;
            std::thread::sleep(Duration::from_millis(15));

            return Ok(()); // just run once
        }
    }

    Ok(())
}
