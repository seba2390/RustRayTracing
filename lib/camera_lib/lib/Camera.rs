use vec_lib::Vector3D;
use vec_lib::DataTypeTraits;

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
    pub fn new(aspect_ratio: T, viewport_height: T) -> Self {
        todo!()
    }
}



// Implementing Vector2D<T> initialization through <T>::new()
impl<T: DataTypeTraits> Vector2D<T>
{
    pub fn new() -> Self {
        Vector2D {x: T::default(), y: T::default()}
    }
}

// Implementing Vector2D<T> initialization through <T>::new()
impl<T: DataTypeTraits> Default for Vector2D<T> {
    fn default() -> Self {
        Vector2D { x: T::default(),
            y: T::default() }
    }
}