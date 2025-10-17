use anyhow;
mod hyperxduocast;
mod lianliinf;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Error, Debug)]
pub enum ParseHexError {
    #[error("bad string, must be hexadecimal in the format 0xRRGGBB or RRGGBB")]
    BadString,
}

impl Into<Rgb> for usize {
    fn into(self) -> Rgb {
        let r = (self >> 16) & 0xFF;
        let g = (self >> 8) & 0xFF;
        let b = self & 0xFF;
        Rgb(r as u8, g as u8, b as u8)
    }
}

impl FromStr for Rgb {
    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);

        if s.len() != 6 {
            return Err(ParseHexError::BadString);
        }

        usize::from_str_radix(s, 16)
            .map(|n| n.into())
            .map_err(|_| ParseHexError::BadString)
    }
}

pub fn setup(color: Rgb) -> anyhow::Result<()> {
    lianliinf::setup()?; // don't use color, just sync with motherboard
    hyperxduocast::setup(color)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn usize_to_rgb() {
        let Rgb(r, g, b): Rgb = 0xff00ee.into();
        assert_eq!(r, 0xff);
        assert_eq!(g, 0x00);
        assert_eq!(b, 0xee);
    }

    #[test]
    fn str_to_rgb() {
        let rgb: Result<Rgb, _> = "0xff00ee".parse();
        assert!(rgb.is_ok());
        let Rgb(r, g, b) = rgb.unwrap();
        assert_eq!(r, 0xff);
        assert_eq!(g, 0x00);
        assert_eq!(b, 0xee);
    }

    #[test]
    fn str_to_rgb_no_prefix() {
        let rgb: Result<Rgb, _> = "ff00ee".parse();
        assert!(rgb.is_ok());
        let Rgb(r, g, b) = rgb.unwrap();
        assert_eq!(r, 0xff);
        assert_eq!(g, 0x00);
        assert_eq!(b, 0xee);
    }

    #[test]
    fn str_to_rgb_bad_length() {
        let rgb: Result<Rgb, _> = "ff00e".parse();
        assert!(rgb.is_err());
    }

    #[test]
    fn str_to_rgb_bad_hex() {
        let rgb: Result<Rgb, _> = "HHJJKK".parse();
        assert!(rgb.is_err());
    }
}
