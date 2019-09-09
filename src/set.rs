use crate::point::Point;
use crate::complex::Part;
use std::ops::Div;

pub struct Set<Unit: Part> {
    points: Vec<Point<Unit>>,
    size: usize,
}

impl<Unit: Part + Div + From<i32>> Set<Unit> {
    pub fn create(size: usize) -> Set<Unit> {
        print!("Starting to allocate\n");
        let mut points = vec![Point::origin(); size * size];
        assert_eq!(size % 4, 0);

        let d: Unit = From::from(size as i32 / 2); // Four wide, but need twice the precision to hit the middle of each pixel
        let start: i32 = (size as i32 / 2) * -2 + 1;
        for i in 0..size {
            for j in 0..size {
                let xi: Unit = (start + 2 * i as i32).into();
                let x: Unit =  xi / d;
                let yi: Unit = (start + 2 * j as i32).into();
                let y: Unit = yi / d;
                points[j + size * i] = Point::from_parts(x, y);
            }
        }
        Set { points, size }
    }
}

impl<Unit: Part> Set<Unit> {
    pub fn iterate_to(self, n: u64) -> Set<Unit> {
        let points = self.points.into_iter();
        let new_points = points.map(|p| p.iterate_to_n(n));
        Set {
            points: new_points.collect(),
            size: self.size,
        }
    }

    pub fn iterate_as_required(self, over: u64) -> Set<Unit> {
        print!("Starting to iterate\n");
        let mut target = 0;
        let mut maximum_non_escaped: u64 = 0;
        let mut new_points = self.points;
        while maximum_non_escaped + over > target {
            target = maximum_non_escaped + over;
            print!("Aiming for {} iterations\n", target);
            let points = new_points.into_iter();
            new_points = points.map(|p| p.iterate_to_n(target)).collect();
            maximum_non_escaped = new_points
                .iter()
                .map(|p| if p.escaped { p.iterations } else { 0 })
                .max()
                .unwrap();
        }
        print!("Saw maximum {} iterations\n", maximum_non_escaped);
        Set {
            points: new_points,
            size: self.size,
        }
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
