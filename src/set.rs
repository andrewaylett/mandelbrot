use std::mem::size_of_val;

use anyhow::{Context, Error};
use itertools::Itertools;
use rayon::prelude::*;

use crate::complex::Complex;
use crate::fix::fix2x61::Fix2x61;
use crate::point::Point;
use crate::zoom_path::Quad;
use std::cmp::min;

pub struct Set {
    points: Vec<Point>,
    power_size: usize,
    centre: Complex,
    radius: Fix2x61,
}

impl Set {
    pub fn subset(&self, quad: &Quad) -> Result<Set, Error> {
        let power_size = self.power_size;
        let radius = self.radius.halve()?;
        let centre = match quad {
            Quad::TopLeft => Complex::new((self.centre.r - radius)?, (self.centre.i - radius)?),
            Quad::TopRight => Complex::new((self.centre.r + radius)?, (self.centre.i - radius)?),
            Quad::BottomLeft => Complex::new((self.centre.r - radius)?, (self.centre.i + radius)?),
            Quad::BottomRight => Complex::new((self.centre.r + radius)?, (self.centre.i + radius)?),
        };
        let mut points = Set::generate_points(power_size, centre, radius)?;

        let size = 1usize << power_size;
        let half_size = 1usize << (power_size - 1);
        let start = match quad {
            Quad::TopLeft => 0,
            Quad::TopRight => half_size,
            Quad::BottomLeft => size * half_size,
            Quad::BottomRight => size * half_size + half_size,
        };
        for x in 0..half_size {
            for y in 0..half_size {
                let old_points_i = start + x + size * y;
                if self.points[old_points_i].escaped {
                    let base = (x * 2 + size * y * 2) as i64;
                    update_if_in_range(&mut points, base, size as i64);
                    update_if_in_range(&mut points, base + 1, size as i64);
                    update_if_in_range(&mut points, base + size as i64, size as i64);
                    update_if_in_range(&mut points, base + size as i64 + 1, size as i64);
                }
            }
        }

        Ok(Set {
            points,
            power_size,
            centre,
            radius,
        })
    }
}

impl Set {
    pub fn create(power_size: usize, centre: Complex, radius: Fix2x61) -> Result<Set, Error> {
        //println!("Starting to allocate");
        let mut points = Set::generate_points(power_size, centre, radius)?;

        let size = 1 << power_size;
        for (i, p) in points.iter_mut().enumerate() {
            if i % size == 0 || i % size == size - 1 || i / size == 0 || i / size == size - 1 {
                p.escape_candidate = true;
            }
        }

        Ok(Set {
            points,
            power_size,
            centre,
            radius,
        })
    }

    fn generate_points(
        power_size: usize,
        centre: Complex,
        radius: Fix2x61,
    ) -> Result<Vec<Point>, Error> {
        assert!(power_size >= 2 && power_size < (size_of_val(&power_size) * 8));
        let size = 1 << power_size;
        let mut points = vec![Point::ORIGIN; size * size];

        // d is half the distance between the points we'll sample.
        // Imagine our square area is made up of size ^ 2 smaller squares.  Our aim is to iterate
        // the middle of each of these smaller squares.
        let d = (radius * Fix2x61::power_of_two(-(power_size as i8))?).truncate()?;
        let d2 = (Fix2x61::two() * d).truncate()?;
        let r_start = ((centre.r - radius)? + d)?;
        let mut i: Result<Fix2x61, Error> = ((centre.i - radius)? + d).map_err(Into::into);
        for each_i in 0..size - 1 {
            let i_ = i.context(each_i)?;
            let mut r: Result<Fix2x61, Error> = Ok(r_start);
            for each_r in 0..size - 1 {
                let r_ = r.context(each_r)?;
                let point = Point::from_parts(&r_, &i_);
                points[each_r + size * each_i] = point;
                r = (r_ + d2).with_context(|| format!("r + d: {:?} + {:?}", r_, d2))
            }
            i = (i_ + d2).with_context(|| format!("i + d: {:?} + {:?}", i_, d2));
        }
        Ok(points)
    }

    pub fn seen_escapes_to(&self) -> u64 {
        self.points
            .iter()
            .max_by_key(|&p| if p.escaped { p.iterations } else { 0 })
            .map(|p| p.iterations)
            .unwrap_or_default()
    }

    pub fn iterate_to(self, n: u64) -> Set {
        let points = self.points.into_iter();
        let new_points = points.map(|p| p.iterate_to_n(n).unwrap());
        Set {
            points: new_points.collect(),
            power_size: self.power_size,
            centre: self.centre,
            radius: self.radius,
        }
    }

    pub fn iterate_as_required(self, min_iter: u64, verbose: bool) -> Result<Set, Error> {
        //println!("Starting to iterate");
        let mut seen_escapes_up_to: u64 = min_iter;
        let mut new_points = self.points;
        let mut new_candidates = true;
        while new_candidates {
            new_candidates = false;
            let target = min(min_iter * 2, seen_escapes_up_to * 2);
            //println!("Aiming for {} iterations", target);
            let points = new_points.into_par_iter();
            new_points = points
                .map(|p: Point| {
                    p.iterate_to_n(target)
                        .with_context(|| format!("Iterating point {:?}", p.value()))
                        .unwrap()
                })
                .collect();
            let size = 1 << self.power_size as i64;
            for i in 0..size * size {
                if new_points[i as usize].escaped {
                    new_candidates |= update_if_in_range(&mut new_points, i + 1, size);
                    new_candidates |= update_if_in_range(&mut new_points, i - 1, size);
                    new_candidates |= update_if_in_range(&mut new_points, i + size + 1, size);
                    new_candidates |= update_if_in_range(&mut new_points, i + size - 1, size);
                    new_candidates |= update_if_in_range(&mut new_points, i - size + 1, size);
                    new_candidates |= update_if_in_range(&mut new_points, i - size - 1, size);
                }
            }
            if let Some(m) =
                new_points
                    .iter()
                    .max_by_key(|&p| if p.escaped { Some(p.iterations) } else { None })
            {
                seen_escapes_up_to = m.iterations;
            }
        }

        if verbose {
            let escaped_iterations = new_points
                .iter()
                .map(|p| if p.escaped { p.iterations } else { 0 })
                .sorted()
                .collect_vec();

            let maximum_escaped_iterations = escaped_iterations.last().unwrap_or(&0);

            let (candidates, not_candidates) = new_points.iter().map(|p| p.escape_candidate).fold(
                (0, 0),
                |(candidates, not_candidates), b: bool| {
                    (
                        candidates + if b { 1 } else { 0 },
                        not_candidates + if b { 0 } else { 1 },
                    )
                },
            );

            println!(
                "Saw maximum {} iterations ({} candidates, {} not candidates) (break at {})",
                maximum_escaped_iterations, candidates, not_candidates, seen_escapes_up_to,
            );
        }
        Ok(Set {
            points: new_points,
            power_size: self.power_size,
            centre: self.centre,
            radius: self.radius,
        })
    }

    pub fn luma_buffer(&self) -> Vec<u8> {
        self.points
            .iter()
            .map(|p| {
                if p.escaped {
                    ((p.iterations % 255) + 1) as u8
                } else {
                    0
                }
            })
            .collect()
    }

    pub fn chroma_buffer(&self) -> Vec<u8> {
        let vga = &crate::colours::VGA_MAP;

        self.points
            .iter()
            .flat_map(|p| {
                if p.escaped {
                    let c = &vga[(p.iterations % 16) as usize];
                    vec![c.0, c.1, c.2]
                } else {
                    vec![0, 0, 0]
                }
            })
            .collect()
    }

    pub fn size(&self) -> u32 {
        1 << self.power_size as u32
    }
}

fn update_if_in_range(new_points: &mut Vec<Point>, r: i64, size: i64) -> bool {
    if r >= 0 && r < size * size {
        let old_val = new_points[r as usize].escape_candidate;
        new_points[r as usize].escape_candidate = true;
        !old_val
    } else {
        false
    }
}
