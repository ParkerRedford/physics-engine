struct Point {
    acceleration: f64,
    speed: f64,
    velocity: (f64, f64, f64),
    position: (f64, f64, f64),
    vector: Vector,
}

impl Point {
    fn add_acceleration<T: ToCartesian>(&mut self, magnitude: f64, direction: &T) {
        let (x, y, z) = direction.to_cartesian();
        self.acceleration.0
    }
}

trait ToCartesian {
    fn to_cartesian(&self) -> (f64, f64, f64);
}

impl ToCartesian for PolarDirection2D {
    fn to_cartesian(&self) -> (f64, f64, f64) {
        let r = self.0;
        let theta = self.1;
        (r * theta.cos, r * theta.sin, 0.0)
    }
}

impl ToCartesian for PolarDirection3D {
    fn to_cartesian(&self) -> (f64, f64, f64) {
        let r = self.0;
        let theta = self.1;
        let phi = self.2;

        (
            r * phi.sin * theta.cos,
            r * phi.sin * theta.sin,
            r * phi.cos
        )
    }
}


struct Vector {
    magnitude: f64,
    direction: Direction
}

