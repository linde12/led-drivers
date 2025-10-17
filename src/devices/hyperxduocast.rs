use hidapi::{HidApi, HidResult};

use super::Rgb;

/// HyperX DuoCast Controller
const VENDOR_ID: u16 = 0x03f0;
const PRODUCT_ID: u16 = 0x098c;
const HYPERX_PACKET_SIZE: usize = 65;

const SAVE_TRANSACTION_START_PKT: [u8; HYPERX_PACKET_SIZE] = {
    let mut buffer: [u8; HYPERX_PACKET_SIZE] = [0u8; HYPERX_PACKET_SIZE];
    buffer[0x01] = 0x04;
    buffer[0x02] = 0x53; // Save to on-board memory
    buffer
};

const TRANSACTION_END_PKT: [u8; HYPERX_PACKET_SIZE] = {
    let mut buffer: [u8; HYPERX_PACKET_SIZE] = [0u8; HYPERX_PACKET_SIZE];
    buffer[0x01] = 0x08;
    buffer[0x3C] = 0x28;
    buffer[0x3D] = 0x01;
    buffer[0x3E] = 0x00;
    buffer[0x3F] = 0xAA;
    buffer[0x40] = 0x55;
    buffer
};

fn build_color_pkt(Rgb(r, g, b): Rgb) -> [u8; HYPERX_PACKET_SIZE] {
    let mut buffer: [u8; HYPERX_PACKET_SIZE] = [0u8; HYPERX_PACKET_SIZE];
    // upper LED array
    buffer[0x01] = 0x81;
    buffer[0x02] = r;
    buffer[0x03] = g;
    buffer[0x04] = b;

    // lower LED array
    buffer[0x05] = 0x81;
    buffer[0x06] = r;
    buffer[0x07] = g;
    buffer[0x08] = b;
    buffer
}

pub fn setup(color: Rgb) -> anyhow::Result<()> {
    let api = HidApi::new()?;

    api.device_list()
        .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
        .ok_or_else(|| anyhow::anyhow!("HyperX DuoCast device not found"))?;

    let device = api.open(VENDOR_ID, PRODUCT_ID)?;

    device.send_feature_report(&SAVE_TRANSACTION_START_PKT)?;
    device.send_feature_report(&build_color_pkt(color))?;
    device.send_feature_report(&TRANSACTION_END_PKT)?;

    Ok(())
}
