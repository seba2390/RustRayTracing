use std::collections::hash_map::VacantEntry;
use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;

pub struct Camera<T: DataTypeTraits>
{
    aspect_ratio: T,
    viewport_height: T,
    viewport_width: T,
    focal_length: T,

    origin: Vector3D<T>,
    horizontal: Vector3D<T>,
    vertical: Vector3D<T>,
    lower_left_corner: Vector3D<T>
}

impl<T: DataTypeTraits> Camera<T>
{
    pub fn new(aspect_ratio: T, viewport_height: T, focal_length: T, origin: Vector3D<T>) -> Self {
        let temp_viewport_height = aspect_ratio * viewport_height;
        Self{ aspect_ratio: aspect_ratio,
              viewport_height: viewport_height,
              viewport_width: temp_viewport_height,
              focal_length: focal_length,

              origin: origin,
              horizontal: Vector3D{x: aspect_ratio * viewport_height, y: T::zero(), z: T::zero()},
              vertical: Vector3D{x: T::zero(), y: viewport_height, z: T::zero()},
              lower_left_corner: origin - Vector3D{x: aspect_ratio * viewport_height, y: T::zero(), z: T::zero()} / T::from(2.0).unwrap()
                     - Vector3D{x: T::zero(), y: viewport_height, z: T::zero()} / T::from(2.0).unwrap()
                     - Vector3D{x: T::zero(), y: T::zero(), z: focal_length} }
    }
}
