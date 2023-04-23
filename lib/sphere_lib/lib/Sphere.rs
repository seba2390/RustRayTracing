use hittable_material_traits::{HitRecord, Hittable, Material};

use vector_lib::Vector3D;
use vector_lib::VectorOperations;
use vector_lib::DataTypeTraits;

use ray_lib::Ray3D;

#[derive(Clone)]
pub struct Sphere<T: DataTypeTraits>
{
    pub center: Vector3D<T>,
    pub radius: T,
}


// Implementing Sphere<T> initialization through <T>::new()
impl<T: DataTypeTraits> Sphere<T>
{
    pub fn new(center: Vector3D<T>, radius: T) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}


impl<T: DataTypeTraits> Hittable<T> for Sphere<T> {
    fn hit(&mut self, ray: &Ray3D<T>, t_min: T, t_max: T, hit_record: &mut HitRecord<T>) -> bool
    {
        let oc: Vector3D<T> = ray.origin - self.center;
        let a: T = ray.direction.inner_product(&ray.direction);
        let b_half: T = oc.inner_product(&ray.direction);
        let c = oc.inner_product(&oc) - self.radius*self.radius;
        let discriminant = b_half*b_half - a*c;
        // Discriminant < 0 -> No intersection of ray and sphere
        if discriminant < T::zero() {
            return false;
        }
        // Intersection of ray and sphere -> determine nearest root in acceptable range.
        let d_sqrt = discriminant.sqrt();
        let mut root = (-b_half - d_sqrt) / a;
        if (root < t_min) || (root > t_max) {
            root = (-b_half + d_sqrt) / a;
            if (root < t_min) || (root > t_max) {
                return false;
            }
        }
        // Intersection occurred - setting hit record of sphere.
        (*hit_record).set_t(root);
        let ray_at = ray.at(root);
        (*hit_record).set_point(ray_at);
        let outwards_normal = (ray_at - &self.center) / self.radius;
        (*hit_record).set_normal_vector(outwards_normal);
        (*hit_record).set_face_normal(&ray, &outwards_normal);
        return true;
    }
}












