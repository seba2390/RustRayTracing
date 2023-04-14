use vector_lib::Vector3D;
use vector_lib::VectorOperations;
use vector_lib::DataTypeTraits;

use ray_lib::Ray3D;

pub struct Sphere<T: DataTypeTraits>
{
    pub center: Vector3D<T>,
    pub radius: T
}


impl<T: DataTypeTraits> Sphere<T>
{
    #[inline(always)]
    pub fn is_hit(&self, ray: &Ray3D<T>) -> bool
    {
        let oc: Vector3D<T> = ray.origin - self.center;
        let a: T = ray.direction.inner_product(&ray.direction);
        let b: T = T::from(2.0).unwrap() * oc.inner_product(&ray.direction);
        let c = oc.inner_product(&oc) - self.radius*self.radius;
        let discriminant = b*b - T::from(4.0).unwrap()*a*c;
        discriminant > T::zero()
    }
}













