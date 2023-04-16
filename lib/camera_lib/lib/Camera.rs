use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;

use ray_lib::Ray3D;


//  The following struct implements a simple camera using the axis-aligned camera
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

/// Computes the ray that passes through the specified pixel on the camera's image plane.
///
/// # Arguments
///
/// * `u` - The horizontal coordinate of the pixel in the range [0, 1], where 0 corresponds to the
///         left edge of the image plane and 1 corresponds to the right edge.
/// * `v` - The vertical coordinate of the pixel in the range [0, 1], where 0 corresponds to the
///         bottom edge of the image plane and 1 corresponds to the top edge.
///
/// # Returns
///
/// A `Ray3D` object originating from the camera's position and traveling in the direction of the
/// specified pixel on the image plane.
impl<T: DataTypeTraits> Camera<T>
{
    pub fn get_ray(&self, u: T, v: T) -> Ray3D<T> {
        Ray3D {origin: self.origin,
               direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin}
    }
}