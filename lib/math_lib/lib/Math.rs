use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;
use vector_lib::VectorOperations;
use ray_lib::Ray3D;

type Color<T> = Vector3D<T>;

#[inline(always)]
pub fn ray_color<T: DataTypeTraits>(ray: &Ray3D<T>) -> Color<T> {
    let unit_direction: Vector3D<T> = ray.direction.unit_vector();
    let t: T = T::from(0.5).unwrap() * (unit_direction.y +  T::one());
    Color{x: T::one(), y:  T::one(), z:  T::one()} * ( T::one() - t) + Color{x: T::from(0.5).unwrap(), y: T::from(0.7).unwrap(), z:  T::one()} * t
}