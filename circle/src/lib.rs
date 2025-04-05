#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let x2 = (self.0 -other.0) * (self.0 - other.0);
        let y2 = (self.1 -other.1) * (self.1 - other.1);
        (x2+y2).sqrt()
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self{
        Circle {
            center: Point(x,y),
            radius,
        }
    }
    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn intersect(self, other:Circle) ->  bool{
        let distance = self.center.distance(other.center);
        distance <= self.radius + other.radius
    }
}
