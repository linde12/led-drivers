use std::time::Duration;

use hidapi::{HidApi, HidResult};

/// HyperX DuoCast Controller
const VENDOR_ID: u16 = 0x03f0;
const PRODUCT_ID: u16 = 0x098c;

const CMD_SET_COLORS: [u8; 10] = [0x00, 0x04, 0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
const COLORS_ALL_WHITE: [u8; 9] = [0x00, 0x81, 0xff, 0xff, 0xff, 0x81, 0xff, 0xff, 0xff];

pub fn setup() -> HidResult<()> {
    let api = HidApi::new()?;

    for device_info in api.device_list() {
        if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
            // TODO: reverse-engineer out how to save to on-board memory if there is any...
            loop {
                let device = api.open(VENDOR_ID, PRODUCT_ID)?;

                device.send_feature_report(&CMD_SET_COLORS)?;
                std::thread::sleep(Duration::from_millis(15));

                device.send_feature_report(&COLORS_ALL_WHITE)?;
                std::thread::sleep(Duration::from_millis(15));
            }
        }
    }

    Ok(())
}
