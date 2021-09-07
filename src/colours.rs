use crate::set::Set;
use lazy_static::lazy_static;
use std::path::Path;
use std::str::FromStr;

use anyhow::{bail, Error};
use image::ColorType;
use std::ops::Deref;

pub struct Rgb(pub u8, pub u8, pub u8);

impl From<&Rgb> for Vec<u8> {
    fn from(r: &Rgb) -> Self {
        vec![r.0, r.1, r.2]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ColourScheme {
    Greyscale,
    Fractint,
    LogGreyscale,
}

struct Greyscale;
static GREYSCALE: Greyscale = Greyscale {};
struct Fractint;
static FRACTINT: Fractint = Fractint {};
struct LogGreyscale;
static LOGGREYSCALE: LogGreyscale = LogGreyscale {};

pub trait ColourSchemeT {
    fn colour_type(&self) -> image::ColorType;
    fn bytes(&self, iterations: u64) -> Vec<u8>;
    fn escaped_bytes(&self) -> Vec<u8>;
}

impl Deref for ColourScheme {
    type Target = dyn ColourSchemeT;

    fn deref(&self) -> &Self::Target {
        match self {
            ColourScheme::Greyscale => &GREYSCALE,
            ColourScheme::Fractint => &FRACTINT,
            ColourScheme::LogGreyscale => &LOGGREYSCALE,
        }
    }
}

impl ColourSchemeT for Greyscale {
    fn colour_type(&self) -> ColorType {
        image::ColorType::Gray(8)
    }

    fn bytes(&self, iterations: u64) -> Vec<u8> {
        vec![((iterations % 255) + 1) as u8]
    }

    fn escaped_bytes(&self) -> Vec<u8> {
        vec![0]
    }
}
impl ColourSchemeT for Fractint {
    fn colour_type(&self) -> ColorType {
        image::ColorType::RGB(8)
    }

    fn bytes(&self, iterations: u64) -> Vec<u8> {
        let modulus = VGA_MAP.len();
        let c = &VGA_MAP[iterations as usize % modulus];
        c.into()
    }

    fn escaped_bytes(&self) -> Vec<u8> {
        vec![0, 0, 0]
    }
}
impl ColourSchemeT for LogGreyscale {
    fn colour_type(&self) -> ColorType {
        image::ColorType::Gray(8)
    }

    fn bytes(&self, iterations: u64) -> Vec<u8> {
        let log_iter = ((iterations as f64).log2() * 64f64) as usize;
        vec![((log_iter % 255) + 1) as u8]
    }

    fn escaped_bytes(&self) -> Vec<u8> {
        vec![0]
    }
}

impl FromStr for ColourScheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fractint" => ColourScheme::Fractint,
            "grey" => ColourScheme::Greyscale,
            "loggrey" => ColourScheme::LogGreyscale,
            _ => bail!("Invalid colour scheme"),
        })
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
    pub static ref VGA_MAP: Vec<Rgb> = vec!(
        Rgb(0, 0, 0),
        Rgb(0, 0, 168),
        Rgb(0, 168, 0),
        Rgb(0, 168, 168),
        Rgb(168, 0, 0),
        Rgb(168, 0, 168),
        Rgb(168, 84, 0),
        Rgb(168, 168, 168),
        Rgb(84, 84, 84),
        Rgb(84, 84, 252),
        Rgb(84, 252, 84),
        Rgb(84, 252, 252),
        Rgb(252, 84, 84),
        Rgb(252, 84, 252),
        Rgb(252, 252, 84),
        Rgb(252, 252, 252),
    );
}
