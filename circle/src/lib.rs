#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let x2 = (self.x -other.x) * (self.x - other.x);
        let y2 = (self.y -other.y) * (self.y - other.y);
        (x2+y2).sqrt()
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self{
        Circle {
            center: Point {
                x: x,
                y:y,
            },
            radius,
        }
    }
    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(self) -> f64 {
        3.142 * self.radius * self.radius
    }
    pub fn intersect(self, other:Circle) ->  bool{
        let distance = self.center.distance(other.center);
        distance <= self.radius + other.radius
    }
}
