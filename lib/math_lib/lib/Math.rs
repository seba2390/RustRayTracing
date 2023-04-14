use color_lib::RGBColor;
use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;
use vector_lib::VectorOperations;
use ray_lib::Ray3D;




/*
This function linearly blends white and blue depending on the height of the y-coordinate
after scaling the rau direction to unit length (so -1.0 < y < 1.0). Because we're looking the
the y height after normalizing the vector, you'll notice a horizontal gradient to the color in
addition to the vertical gradient.

Did a standard graphics trick of scaling that to 0.0 <= t <= 1.0. When t = 1.0 we get blue, and when
t = 0.0 we get white. In between we get a blend. This forms a 'linear blend', or
'linear interpolation' between to points. A linear blending interpolation always has the form:

                         BlendedValue = (1.0 - t) * StartVal + t * EndVal,

with 't' going from zero to one.
 */
#[inline(always)]
pub fn ray_color<T: DataTypeTraits>(ray: &Ray3D<T>) -> RGBColor<T> {
    let unit_direction: Vector3D<T> = ray.direction.unit_vector();
    let t: T = T::from(0.5).unwrap() * (unit_direction.y +  T::one());
    RGBColor{R: T::one(), G:  T::one(), B:  T::one()} * ( T::one() - t) + RGBColor{R: T::from(0.5).unwrap(), G: T::from(0.7).unwrap(), B:  T::one()} * t
}