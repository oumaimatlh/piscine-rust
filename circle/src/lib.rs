#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64
}

impl Circle {
    pub fn new(x: f64, y:f64, radius:f64) -> Self {
        Self {
            center : Point(x, y),
            radius : radius
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn intersect(&self, cercle: Self) -> bool {
        let d = self.center.distance(cercle.center);
        d <= self.radius + cercle.radius 
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, point: Self) -> f64 {
        ((point.0 - self.0).powi(2) + (point.1 - self.1).powi(2)).sqrt()
    }
}
