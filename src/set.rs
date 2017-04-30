use point::Point;

pub struct Set {
    points: Vec<Point>
}

impl Set {
    pub fn create() -> Set {
        let mut points = vec![Point::new(0.0,0.0); 150*150];
        let d = 4.0/150.0;
        for i in 0..150 {
            for j in 0..150 {
                let x = d * (i as f64 - 74.5);
                let y = d * (j as f64 - 74.5);
                points[j + 150*i] = Point::new(x,y);
            }
        }
        Set { points }
    }

    pub fn iterate_to(self, n: u64) -> Set {
        let points = self.points.into_iter();
        let new_points = points.map(|p| p.iterate_to_n(n));
        Set { points: new_points.collect() }
    }

    pub fn luma_buffer(&self) -> Vec<u8> {
        self.points.iter().map(|p| {
            if p.escaped {
                ((p.iterations % 255) + 1) as u8
            } else {
                0 as u8
            }
        }).collect()
    }
}
