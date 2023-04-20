use ray_lib::Ray3D;

use vector_lib::{Vector3D, VectorOperations};
use vector_lib::DataTypeTraits;

use color_lib::RGBColor;

////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////// MATERIAL TRAIT ////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Material<T: DataTypeTraits> {
    fn scatter(
        &self,
        ray_in: &Ray3D<T>,
        rec: &HitRecord<T>,
        attenuation: &mut RGBColor<T>,
        scattered: &mut Ray3D<T>,
    ) -> bool;
}


pub struct NoMaterial;
impl<T: DataTypeTraits> Material<T> for NoMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray3D<T>,
        _rec: &HitRecord<T>,
        _attenuation: &mut RGBColor<T>,
        _scattered: &mut Ray3D<T>,
    ) -> bool {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////// HITRECORD STRUCT ///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// Struct for recording ray collisions
#[derive(Clone)]
pub struct HitRecord<T: DataTypeTraits> {
    point: Vector3D<T>,
    normal_vector: Vector3D<T>,
    t: T,
    front_face: bool, // True if ray is hitting from outside object, false o.w.
}


////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////// SPECIAL IMPLS ////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: DataTypeTraits> HitRecord<T> {
    pub fn set_face_normal(&mut self, ray: &Ray3D<T>, outwards_normal: &Vector3D<T>) {
        // Positive inner product -> ray is inside the object, o.w ray is outside the object
        self.front_face = ray.direction.inner_product(&outwards_normal) < T::zero();
        self.normal_vector = if self.front_face { outwards_normal.clone() } else { outwards_normal.clone() * (-T::one()) };
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////// INITIALIZATION IMPL  /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: DataTypeTraits> Default for HitRecord<T> {
    fn default() -> Self {
        HitRecord {
            point: Vector3D::default(),
            normal_vector: Vector3D::default(),
            t: T::default(),
            front_face: bool::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////// GETTER & SETTER IMPLS ////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// =========================================== GETTERS ========================================== //

impl<T: DataTypeTraits> HitRecord<T> {
    pub fn get_point(&self) -> Vector3D<T> {
        self.point.clone()
    }
}
impl<T: DataTypeTraits> HitRecord<T> {
    pub fn get_normal_vector(&self) -> Vector3D<T> {
        self.normal_vector.clone()
    }
}

impl<T: DataTypeTraits> HitRecord<T> {
    pub fn get_t(&self) -> T {
        self.t.clone()
    }
}

// =========================================== SETTERS ========================================== //

impl<T: DataTypeTraits> HitRecord<T> {
    pub fn set_point(&mut self, point: Vector3D<T>) {
        self.point = point.clone();
    }
}
impl<T: DataTypeTraits> HitRecord<T> {
    pub fn set_normal_vector(&mut self, normal_vector: Vector3D<T>){
        self.normal_vector = normal_vector.clone();
    }
}

impl<T: DataTypeTraits> HitRecord<T> {
    pub fn set_t(&mut self, t: T){
        self.t = t.clone();
    }
}



////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// SPECIAL TRAIT FOR OBJECTS //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: implement this trait for any new shape created in scene ObjectType enum
pub trait Hittable<T: DataTypeTraits>
{
    fn hit(&mut self, ray: &Ray3D<T>, t_min: T, t_max: T, hit_record: & mut HitRecord<T>) -> bool;
}