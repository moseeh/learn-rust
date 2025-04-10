pub mod areas_volumes; // Import the module containing area and volume calculation functions
pub use areas_volumes::*; // Re-export everything from the module for use in this file

// Determines if a given number of 2D shapes can fit within a 2D container of size x * y
pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let max_size = x * y; // Calculate total available area
    let size;
    match objects {
        GeometricalShapes::Square => size = square_area(a) as f64, // Area of square with side a
        GeometricalShapes::Circle => size = circle_area(a), // Area of circle with radius a
        GeometricalShapes::Rectangle => size = rectangle_area(a, b) as f64, // Area of rectangle a × b
        GeometricalShapes::Triangle => size = triangle_area(a, b), // Area of triangle with base a and height b
    }
    times as f64 * size <= max_size as f64 // Check if total area of all shapes fits within container
}

// Determines if a given number of 3D volumes can fit within a 3D container of size x * y * z
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let max_size = x * y * z; // Calculate total available volume
    let size;
    match objects {
        GeometricalVolumes::Cube => size = cube_volume(a) as f64, // Volume of cube with side a
        GeometricalVolumes::Sphere => size = sphere_volume(a), // Volume of sphere with radius a
        GeometricalVolumes::Parallelepiped => size = parallelepiped_volume(a, b, c) as f64, // Volume of parallelepiped
        GeometricalVolumes::Pyramid => size = triangular_pyramid_volume(triangle_area(a, b), c), // Volume of triangular pyramid
        GeometricalVolumes::Cone => size = cone_volume(a, b), // Volume of cone with radius a and height b
    }
    times as f64 * size <= max_size as f64 // Check if total volume of all objects fits within container
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_volumes_shapes() {
        // Test cases with zero objects to fit — should always return true
        assert_eq!(true, area_fit(2, 5, GeometricalShapes::Circle, 0, 2, 1));
        assert_eq!(true, area_fit(2, 2, GeometricalShapes::Rectangle, 0, 6, 10));
        assert_eq!(
            true,
            volume_fit(2, 5, 3, GeometricalVolumes::Cone, 0, 1, 1, 1)
        );
        assert_eq!(
            true,
            volume_fit(3, 5, 7, GeometricalVolumes::Parallelepiped, 0, 2, 6, 3)
        );
    }

    #[test]
    fn equal_size() {
        // Test cases where object size exactly matches container size
        assert_eq!(true, area_fit(2, 5, GeometricalShapes::Square, 1, 2, 5));
        assert_eq!(
            true,
            volume_fit(3, 1, 4, GeometricalVolumes::Cube, 1, 1, 3, 4)
        );
    }

    #[test]
    fn all_shapes() {
        // Test various shapes with different counts and dimensions
        assert_eq!(false, area_fit(3, 5, GeometricalShapes::Circle, 2, 2, 0));
        assert_eq!(true, area_fit(8, 6, GeometricalShapes::Triangle, 5, 5, 2));
        assert_eq!(true, area_fit(7, 3, GeometricalShapes::Rectangle, 2, 2, 4));
        assert_eq!(true, area_fit(5, 5, GeometricalShapes::Square, 1, 2, 4));
    }

    #[test]
    fn all_volumes() {
        // Test various volumes with different counts and dimensions
        assert_eq!(
            true,
            volume_fit(5, 6, 3, GeometricalVolumes::Cube, 2, 3, 3, 4)
        );
        assert_eq!(
            false,
            volume_fit(7, 4, 4, GeometricalVolumes::Cone, 1, 8, 2, 0)
        );
        assert_eq!(
            true,
            volume_fit(2, 5, 3, GeometricalVolumes::Sphere, 1, 1, 1, 1)
        );
        assert_eq!(
            false,
            volume_fit(2, 5, 3, GeometricalVolumes::Parallelepiped, 31, 1, 1, 1)
        );
        assert_eq!(
            true,
            volume_fit(7, 5, 3, GeometricalVolumes::Pyramid, 3, 3, 2, 1)
        );
    }
}