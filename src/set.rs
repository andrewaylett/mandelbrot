use crate::complex::{Complex, Fix2x61, FixResult};
use crate::point::Point;
use anyhow::{Context, Error};
use std::mem::size_of_val;

pub struct Set {
    points: Vec<Point>,
    size: usize,
}

impl Set {
    pub fn create(size_power_of_two: usize) -> Result<Set, Error> {
        print!("Starting to allocate\n");
        assert!(
            size_power_of_two >= 2 && size_power_of_two < (size_of_val(&size_power_of_two) * 8)
        );
        let size = 1 << size_power_of_two;
        let mut points = vec![Point::origin(); size * size];
        assert_eq!(size % 4, 0);

        let centre = Complex::new(Fix2x61::zero(), Fix2x61::zero());
        let radius = Fix2x61::two();

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
                points[each_r + size * each_i] = Point::from_parts(&r_, &i_);
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

    pub fn iterate_as_required(self, over: u64) -> Result<Set, Error> {
        print!("Starting to iterate\n");
        let mut target = 0;
        let mut maximum_non_escaped: u64 = 0;
        let mut new_points = self.points;
        while maximum_non_escaped + over > target {
            target = maximum_non_escaped + over;
            print!("Aiming for {} iterations\n", target);
            let points = new_points.into_iter();
            new_points = points
                .map(|p| {
                    p.iterate_to_n(target)
                        .with_context(|| format!("Iterating point {:?}", p.value()))
                        .unwrap()
                })
                .collect();
            maximum_non_escaped = new_points
                .iter()
                .map(|p| if p.escaped { p.iterations } else { 0 })
                .max()
                .unwrap();
        }
        print!("Saw maximum {} iterations\n", maximum_non_escaped);
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
                    0 as u8
                }
            })
            .collect()
    }

    pub fn size(&self) -> u32 {
        self.size as u32
    }
}
