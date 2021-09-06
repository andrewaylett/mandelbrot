use std::mem::size_of_val;

use anyhow::{Context, Error};
use itertools::Itertools;
use rayon::prelude::*;

use crate::complex::Complex;
use crate::fix::fix2x61::Fix2x61;
use crate::point::Point;
use std::cmp::{max, min};

pub struct Set {
    points: Vec<Point>,
    size: usize,
}

impl Set {
    pub fn create(
        size_power_of_two: usize,
        centre: Complex,
        radius: Fix2x61,
    ) -> Result<Set, Error> {
        //println!("Starting to allocate");
        assert!(
            size_power_of_two >= 2 && size_power_of_two < (size_of_val(&size_power_of_two) * 8)
        );
        let size = 1 << size_power_of_two;
        let mut points = vec![Point::ORIGIN; size * size];
        assert_eq!(size % 4, 0);

        // d is half the distance between the points we'll sample.
        // Imagine our square area is made up of size ^ 2 smaller squares.  Our aim is to iterate
        // the middle of each of these smaller squares.
        let d = (radius * Fix2x61::power_of_two(1 - (size_power_of_two as i8))?).truncate()?;
        let r_start = ((centre.r - radius)? + d)?;
        let mut i: Result<Fix2x61, Error> = ((centre.i - radius)? + d).map_err(Into::into);
        for each_i in 0..size - 1 {
            let i_ = i.context(each_i)?;
            let mut r: Result<Fix2x61, Error> = Ok(r_start);
            for each_r in 0..size - 1 {
                let r_ = r.context(each_r)?;
                let mut point = Point::from_parts(&r_, &i_);
                if each_i == 0 || each_i == size - 1 || each_r == 0 || each_r == size - 1 {
                    point.escape_candidate = true;
                }
                points[each_r + size * each_i] = point;
                r = (r_ + d).with_context(|| format!("r + d: {:?} + {:?}", r_, d))
            }
            i = (i_ + d).with_context(|| format!("i + d: {:?} + {:?}", i_, d));
        }
        Ok(Set { points, size })
    }
}

impl Set {
    pub fn iterate_to(self, n: u64) -> Set {
        let points = self.points.into_iter();
        let new_points = points.map(|p| p.iterate_to_n(n).unwrap());
        Set {
            points: new_points.collect(),
            size: self.size,
        }
    }

    pub fn iterate_as_required(
        self,
        min_iter: u64,
        over: u64,
        verbose: bool,
    ) -> Result<Set, Error> {
        //println!("Starting to iterate");
        let mut target = 0;
        let mut seen_escapes_up_to: u64 = min_iter;
        let mut new_points = self.points;
        let mut new_candidates = true;
        while seen_escapes_up_to + over > target || new_candidates {
            new_candidates = false;
            target = max(target + over / 4, seen_escapes_up_to + over / 2)
                .min(seen_escapes_up_to + over);
            //println!("Aiming for {} iterations", target);
            let points = new_points.into_par_iter();
            new_points = points
                .map(|p: Point| {
                    p.iterate_to_n(target)
                        .with_context(|| format!("Iterating point {:?}", p.value()))
                        .unwrap()
                })
                .collect();
            let size = self.size as i64;
            for i in 0..size * size {
                let update_if_in_range = |new_points: &mut Vec<Point>, r: i64| {
                    if r >= 0 && r < size * size {
                        let old_val = new_points[r as usize].escape_candidate;
                        new_points[r as usize].escape_candidate = true;
                        !old_val
                    } else {
                        false
                    }
                };

                if new_points[i as usize].escaped {
                    new_candidates |= update_if_in_range(&mut new_points, i + 1);
                    new_candidates |= update_if_in_range(&mut new_points, i - 1);
                    new_candidates |= update_if_in_range(&mut new_points, i + size + 1);
                    new_candidates |= update_if_in_range(&mut new_points, i + size - 1);
                    new_candidates |= update_if_in_range(&mut new_points, i - size + 1);
                    new_candidates |= update_if_in_range(&mut new_points, i - size - 1);
                }
            }
            let a = new_points.iter().flat_map(|p| -> Option<i64> {
                if p.escaped {
                    Some(-(p.iterations as i64))
                } else {
                    None
                }
            });
            let len = min(self.size / 2, a.clone().count() / 5000);

            seen_escapes_up_to = (-a.k_smallest(len).max().unwrap_or(0)) as u64;
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
                "Saw maximum {} iterations ({} candidates, {} not candidates) (break at {}, then {:?})",
                maximum_escaped_iterations,
                candidates,
                not_candidates,
                seen_escapes_up_to,
                escaped_iterations
                    .iter()
                    .filter(|&v| v > &seen_escapes_up_to)
                    .collect_vec()
            );
        }
        Ok(Set {
            points: new_points,
            size: self.size,
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

    pub fn size(&self) -> u32 {
        self.size as u32
    }
}
