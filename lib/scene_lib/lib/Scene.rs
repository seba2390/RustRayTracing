
use vector_lib::DataTypeTraits;

use ray_lib::Ray3D;

use hittable_traits::{HitRecord, Hittable};

////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////// STRUCT DEFINITION //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Scene<T>
{
    pub objects: Vec<Box<dyn Hittable<T> + Send + Sync>>
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////// STRUCT IMPL DEFINITIONS ///////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// Implementing Scene<T> initialization through <T>::new()
impl<T: DataTypeTraits> core::default::Default for Scene<T> {
    fn default() -> Self {
        Scene {objects: Vec::<Box<dyn Hittable<T> + Send + Sync>>::default() }
    }
}

impl<T: DataTypeTraits> Scene<T> {
    fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl<T: DataTypeTraits> Scene<T> {
    pub fn add(& mut self, object: Box<dyn Hittable<T> + Send + Sync>) {
        self.objects.push(object);
    }
}

impl<T: DataTypeTraits> Hittable<T> for Scene<T> {
    fn hit(&mut self, ray: &Ray3D<T>, t_min: T, t_max: T, hit_record: &mut HitRecord<T>) -> bool
    {
        let mut hit_record_temp = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter_mut() {
            if obj.hit(ray, t_min, closest_so_far, & mut hit_record_temp){
                hit_anything = true;
                closest_so_far = hit_record_temp.get_t();
                *hit_record = hit_record_temp;
            }
        }
        hit_anything

    }
}