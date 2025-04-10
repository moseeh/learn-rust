// Enum representing 2D shapes
pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

// Enum representing 3D volumes
pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    Pyramid,
    Parallelepiped,
}

// Calculates the area of a square given one side
pub fn square_area(side: usize) -> usize {
    side.pow(2)
}

// Calculates the area of a triangle given base and height
pub fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

// Calculates the area of a circle given the radius
pub fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

// Calculates the area of a rectangle given two sides
pub fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

// Calculates the volume of a cube given the side length
pub fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

// Calculates the volume of a sphere given the radius
pub fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

// Calculates the volume of a triangular pyramid given base area and height
pub fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

// Calculates the volume of a rectangular parallelepiped (box) given 3 sides
pub fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

// Calculates the volume of a cone given base radius and height
pub fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}