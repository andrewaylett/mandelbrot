use crate::point::Point;

pub struct Set {
    points: Vec<Point>,
    size: usize,
}

impl Set {
    pub fn create(size: usize) -> Set {
        let mut points = vec![Point::new(0.0, 0.0); size * size];
        let d = 4.0 / (size as f64);
        let start = (size as f64 / 2.0) + 0.5;
        for i in 0..size {
            for j in 0..size {
                let x = d * (i as f64 - start);
                let y = d * (j as f64 - start);
                points[j + size * i] = Point::new(x, y);
            }
        }
        Set { points, size }
    }

    pub fn iterate_to(self, n: u64) -> Set {
        let points = self.points.into_iter();
        let new_points = points.map(|p| p.iterate_to_n(n));
        Set {
            points: new_points.collect(),
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
