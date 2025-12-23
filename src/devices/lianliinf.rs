use hidapi::HidApi;

use super::SetupError;

/// LianLi-SL-infinity-v0.8
const VENDOR_ID: u16 = 0x0cf2;
const PRODUCT_ID: u16 = 0xa102;
const CMD_MB_SYNC_ON: [u8; 4] = [0xe0, 0x10, 0x61, 0x01];

pub fn setup() -> Result<(), SetupError> {
    let api = HidApi::new()?;

    api.device_list()
        .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
        .ok_or_else(|| SetupError::DeviceNotFound("LianLi SL Infinity"))?;

    // device found, turn on motherboard sync
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;
    device.write(&CMD_MB_SYNC_ON)?;

    Ok(())
}
