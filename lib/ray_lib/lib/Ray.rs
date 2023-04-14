use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;

pub struct Ray3D<T: DataTypeTraits> {
    pub origin: Vector3D<T>,
    pub direction: Vector3D<T>
}

impl<T: DataTypeTraits> Ray3D<T> {
    pub fn at(self, t: T) -> Vector3D<T> {
        &self.origin + &self.direction * t
    }
}
