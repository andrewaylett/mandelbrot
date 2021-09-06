use anyhow::{bail, Context, Error};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Quad {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct ZoomPath(pub Vec<Quad>);

impl FromStr for ZoomPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path: Result<Vec<_>, _> = s
            .split(',')
            .enumerate()
            .map(|(i, arg)| -> Result<Quad, Error> {
                let v = u8::from_str(arg).with_context(|| {
                    format!(
                        "element {} is {} but should be a number from 1 to 4",
                        i, arg
                    )
                })?;
                Ok(match v {
                    1 => Quad::TopLeft,
                    2 => Quad::TopRight,
                    3 => Quad::BottomLeft,
                    4 => Quad::BottomRight,
                    _ => bail!("element {} is out of range (1-4): {}", i, v),
                })
            })
            .collect();
        Ok(ZoomPath(path?))
    }
}
