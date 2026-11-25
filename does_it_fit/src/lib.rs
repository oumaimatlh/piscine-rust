pub mod areas_volumes;
use areas_volumes::{GeometricalShapes, GeometricalVolumes}; 

pub fn area_fit(
    (x, y): (usize, usize), //width && height d rectangle
    kind: areas_volumes::GeometricalShapes,//type d forme ; triangle, carre ect ...Enum 
    times: usize, //how many times place a shape 
    (a, b): (usize, usize), //1er dimension et 2eme dimension d forme 
) -> bool {
    let area_rectangle = areas_volumes::rectangle_area(x, y);

    let area_kind = match kind {
         areas_volumes::GeometricalShapes::Square =>  areas_volumes::square_area(a) as usize,
         areas_volumes::GeometricalShapes::Circle =>  areas_volumes::circle_area(a) as usize,
         areas_volumes::GeometricalShapes::Triangle =>  areas_volumes::triangle_area(a, b) as usize,
         areas_volumes::GeometricalShapes::Rectangle =>  areas_volumes::rectangle_area(a, b) as usize , 
    };

    if times * area_kind <= area_rectangle {
        return true; 
    }
    return false; 
}


//area-fit:
    /*
        Est-ce-que je peux placer times formes geometriques ds un rectangle de dimension x, y 
        on doit connaitre la surface dispo (du rectangle); la surface occupe par une forme ; le nombre de formes que je placer 
    */
pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_volume = areas_volumes::parallelepiped_volume(x, y, z) as f64;

    let shape_volume = match kind {
        areas_volumes::GeometricalVolumes::Cube => {
            areas_volumes::cube_volume(a) as f64
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            areas_volumes::sphere_volume(a)
        }
        areas_volumes::GeometricalVolumes::Cone => {
            areas_volumes::cone_volume(a, b)
        }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };

    (times as f64 * shape_volume) <= box_volume
}