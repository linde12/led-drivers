use hidapi::{HidApi, HidResult};

/// LianLi-SL-infinity-v0.8
const VENDOR_ID: u16 = 0x0cf2;
const PRODUCT_ID: u16 = 0xa102;
const CMD_MB_SYNC_ON: [u8; 4] = [0xe0, 0x10, 0x61, 0x01];

pub fn setup() -> HidResult<()> {
    let api = HidApi::new()?;

    for device in api.device_list() {
        if device.vendor_id() == VENDOR_ID && device.product_id() == PRODUCT_ID {
            // device found, turn on motherboard sync
            let device = api.open(VENDOR_ID, PRODUCT_ID)?;
            device.write(&CMD_MB_SYNC_ON)?;
        }
    }

    Ok(())
}
