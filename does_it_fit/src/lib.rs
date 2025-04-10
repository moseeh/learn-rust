use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    // Calculate the total area of the rectangle
    let total_area = x * y;
    
    // Calculate the area of a single object based on its type
    let single_object_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };
    
    // Calculate the total area needed for all objects
    let needed_area = single_object_area * times as f64;
    
    // Check if the objects can fit
    needed_area <= total_area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    // Calculate the total volume of the box
    let total_volume = x * y * z;
    
    // Calculate the volume of a single object based on its type
    let single_object_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Pyramid => {
            // For Pyramid, 'a' is base_area and 'b' is height
            // We need to calculate the base area as a triangle
            let base_area = a as f64; // a is already the base_area according to instructions
            areas_volumes::triangular_pyramid_volume(base_area, b)
        },
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };
    
    // Calculate the total volume needed for all objects
    let needed_volume = single_object_volume * times as f64;
    
    // Check if the objects can fit
    needed_volume <= total_volume as f64
}