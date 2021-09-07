use crate::set::Set;
use lazy_static::lazy_static;
use std::path::Path;
use std::str::FromStr;

use anyhow::{bail, Error};

pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug, Copy, Clone)]
pub enum ColourScheme {
    Greyscale,
    Fractint,
}

impl FromStr for ColourScheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fractint" => ColourScheme::Fractint,
            "grey" => ColourScheme::Greyscale,
            _ => bail!("Invalid colour scheme"),
        })
    }
}

impl ColourScheme {
    fn colour_type(&self) -> image::ColorType {
        match &self {
            ColourScheme::Greyscale => image::ColorType::Gray(8),
            ColourScheme::Fractint => image::ColorType::RGB(8),
        }
    }

    fn bytes(&self, iterations: u64) -> Vec<u8> {
        match &self {
            ColourScheme::Greyscale => {
                vec![((iterations % 255) + 1) as u8]
            }
            ColourScheme::Fractint => {
                let c = &VGA_MAP[(iterations % 16) as usize];
                vec![c.0, c.1, c.2]
            }
        }
    }

    fn escaped_bytes(&self) -> Vec<u8> {
        match &self {
            ColourScheme::Greyscale => {
                vec![0]
            }
            ColourScheme::Fractint => {
                vec![0, 0, 0]
            }
        }
    }
}

impl Set {
    pub fn render_to_file(&self, scheme: &ColourScheme, filename: &str) -> std::io::Result<()> {
        let buffer: Vec<u8> = self
            .points
            .iter()
            .flat_map(|p| {
                if p.escaped {
                    scheme.bytes(p.iterations)
                } else {
                    scheme.escaped_bytes()
                }
            })
            .collect();

        image::save_buffer(
            &Path::new(filename),
            &buffer[..],
            self.size(),
            self.size(),
            scheme.colour_type(),
        )
    }
}

lazy_static! {
    // Ref https://svn.fractint.net/trunk/fractint/maps/default.map
    pub static ref VGA_MAP: Vec<RGB> = vec!(
        RGB(0, 0, 0),
        RGB(0, 0, 168),
        RGB(0, 168, 0),
        RGB(0, 168, 168),
        RGB(168, 0, 0),
        RGB(168, 0, 168),
        RGB(168, 84, 0),
        RGB(168, 168, 168),
        RGB(84, 84, 84),
        RGB(84, 84, 252),
        RGB(84, 252, 84),
        RGB(84, 252, 252),
        RGB(252, 84, 84),
        RGB(252, 84, 252),
        RGB(252, 252, 84),
        RGB(252, 252, 252),
    );
}
