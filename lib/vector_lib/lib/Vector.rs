use rand::distributions::{Distribution, Uniform};

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////// DATA TRAIT DEFINITIONS ///////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////


// 1. Create a new trait
// Only supports f32 and f64 types in Vector (wanna have well-defined length without typecast)
pub trait DataTypeTraits:
        num_traits::Float + std::fmt::Display + std::fmt::Debug +
        std::marker::Copy + std::default::Default + num_traits::Zero +
        num_traits::One + rand::distributions::uniform::SampleUniform +
        std::clone::Clone + num_traits::float::FloatConst + std::marker::Send{
        // we'd usually add more functions in this block,
        // but in this case we don't need any more.
}

// 2. Implement it
impl<T> DataTypeTraits for T
    where T: num_traits::Float + std::fmt::Display + std::fmt::Debug +
    std::marker::Copy + std::default::Default + num_traits::Zero +
    num_traits::One + rand::distributions::uniform::SampleUniform +
    std::clone::Clone + num_traits::float::FloatConst + std::marker::Send{
    // Nothing to implement, since T already supports the other traits.
    // It has the functions it needs already
}


////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////// VECTOR TRAIT DEFINITIONS /////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////


pub trait VectorOperations<T: DataTypeTraits>: std::clone::Clone
{
    // Vector specific (`#[inline]` is ignored on function prototypes)
    fn inner_product(&self, other: &Self) -> T;

    // Generic for all vectors
    #[inline(always)]
    fn norm(&self) -> T {
        self.inner_product(self).sqrt()
    }

    // Generic for all vectors
    #[inline(always)]
    fn normalize(&mut self)
        where Self: std::ops::Div<T, Output=Self> + core::marker::Sized + std::marker::Copy
    {
        let norm = self.norm();
        *self = *self / norm;
    }

    // Generic for all vectors
    #[inline(always)]
    fn unit_vector(&self) -> Self
        where Self: Sized + std::ops::Div<T, Output=Self>
    {
        self.clone() / self.norm()
    }


    // Generic for all vectors
    #[inline(always)]
    fn project(&self, other: &Self) -> Self
        where Self: core::marker::Sized + std::ops::Div<T, Output=Self> + std::ops::Mul<T, Output=Self> + std::marker::Copy{
        let norm = other.norm();
        ((*other) / (norm*norm)) * self.inner_product(other)
    }

}



////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// STRUCT DEFINITIONS //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(PartialEq, Clone, Copy)]
pub struct Vector2D<T: DataTypeTraits> {
    pub x: T,
    pub y: T
}

#[derive(PartialEq, Clone, Copy)]
pub struct Vector3D<T: DataTypeTraits> {
    pub x: T,
    pub y: T,
    pub z: T
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////// IMPL DEFINITIONS //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/*
    Note:
     1) #[inline(always)] is an attribute in Rust that hints the compiler to always inline a
        function or method when it's called, rather than generating a separate function call.
        This can improve performance by reducing function call overhead and enabling the compiler
        to optimize the code more effectively.

     2) The 'a part in Rust syntax is called a lifetime parameter. It is used to specify how long a
        reference should live, and is typically used when working with references to avoid issues
        like dangling references or use-after-free errors.
        When a lifetime parameter is included in a function or struct definition, it means that any
        references used within that function or struct must have a lifetime that is at least as
        long as the specified lifetime parameter.
 */



/******************************************** 2D VECTOR *******************************************/


// Implementing fmt::Display, which uses the {} print marker
impl<T: DataTypeTraits> std::fmt::Display for Vector2D<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write the first and second element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}", self.x, self.y)
    }
}

// Implementing fmt::Debug, which uses the {:?} print marker
impl<T: DataTypeTraits>  std::fmt::Debug for Vector2D<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector2D")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
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


// Implementing Vector2D<T> with zeros initialization through <T>::zeros()
impl<T: DataTypeTraits> Vector2D<T>
{
    pub fn zeros() -> Self {
        Vector2D {x: T::zero(), y: T::zero()}
    }
}

// Implementing Vector2D<T> with ones initialization through <T>::ones()
impl<T: DataTypeTraits> Vector2D<T>
{
    pub fn ones() -> Self {
        Vector2D {x: T::one(), y: T::one()}
    }
}


// Implementing Vector2D<T> with random uniform in [low,high[ initialization through <T>::random_uniform()
impl<T: DataTypeTraits> Vector2D<T>
{
    pub fn random_uniform(lower_bound: T, upper_bound: T) -> Vector2D<T> {
        if upper_bound <= lower_bound {
            panic!("Upper bound cannot be less than or equal to lower bound");
        }
        let distribution = Uniform::try_from(lower_bound..upper_bound).unwrap();
        let mut rng = rand::thread_rng();
        Vector2D { x: distribution.sample(&mut rng),
                   y: distribution.sample(&mut rng)}
    }
}


// Implementing Vector2D<T> with random uniform in [low,high[ initialization through <T>::fast_random_uniform()
impl<T: DataTypeTraits> Vector2D<T>
{
    pub fn fast_random_uniform(lower_bound: T, upper_bound: T) -> Vector2D<T> {
        if upper_bound <= lower_bound {
            panic!("Upper bound cannot be less than or equal to lower bound");
        }
        let rng = fastrand::Rng::new();
        if std::mem::size_of::<T>() == std::mem::size_of::<f32>() {
            Vector2D { x: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound),
                       y: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound)}
        }
        else {
            Vector2D { x: lower_bound + T::from(rng.f64()).unwrap() * (upper_bound - lower_bound),
                       y: lower_bound + T::from(rng.f64()).unwrap() * (upper_bound - lower_bound)}
        }
    }
}


//++++++++++++++++++++++++++++++++++++++++ Addition ++++++++++++++++++++++++++++++++++++++++++++ //
// Implementing Vector2D<T> + Vector2D<T> -> Vector2D<T> type addition
impl<T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<Vector2D<T>> for Vector2D<T> {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self { x: self.x + other.x,
               y: self.y + other.y }
    }
}
// Implementing &Vector2D<T> + &Vector2D<T> -> Vector2D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn add(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x + other.x,
                   y: self.y + other.y }
    }
}
// Implementing Vector2D<T> + &Vector2D<T> -> Vector2D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a Vector2D<T>> for Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn add(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x + other.x,
                   y: self.y + other.y }
    }
}
// Implementing &Vector2D<T> + Vector2D<T> -> Vector2D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn add(self, other: Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x + other.x,
                   y: self.y + other.y }
    }
}


//----------------------------------------- Subtraction ----------------------------------------- //

// Implementing Vector2D<T> - Vector2D<T> -> Vector2D<T> type subtraction
impl<T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<Vector2D<T>> for Vector2D<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self { x: self.x - other.x,
               y: self.y - other.y }
    }
}
// Implementing &Vector2D<T> - &Vector2D<T> -> Vector2D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn sub(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x - other.x,
                   y: self.y - other.y }
    }
}
// Implementing Vector2D<T> - &Vector2D<T> -> Vector2D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a Vector2D<T>> for Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn sub(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x - other.x,
                   y: self.y - other.y }
    }
}
// Implementing &Vector2D<T> - Vector2D<T> -> Vector2D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn sub(self, other: Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x - other.x,
                   y: self.y - other.y }
    }
}


//********************************** Entry-wise multiplication ***********************************//

// Implementing Vector2D<T> * Vector2D<T> -> Vector2D<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<Vector2D<T>> for Vector2D<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self { x: self.x * other.x,
               y: self.y * other.y }
    }
}
// Implementing &Vector2D<T> * &Vector2D<T> -> Vector2D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn mul(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x * other.x,
                   y: self.y * other.y }
    }
}
// Implementing Vector2D<T> * &Vector2D<T> -> Vector2D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a Vector2D<T>> for Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn mul(self, other: &Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x * other.x,
                   y: self.y * other.y }
    }
}
// Implementing &Vector2D<T> * Vector2D<T> -> Vector2D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;
    #[inline(always)]
    fn mul(self, other: Vector2D<T>) -> Self::Output {
        Vector2D { x: self.x * other.x,
                   y: self.y * other.y }
    }
}


//******************************* Right Side Scalar multiplication *******************************//

// Implementing Vector2D<T> * T -> Vector2D<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for Vector2D<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        Self { x: self.x * scalar,
               y: self.y * scalar }
    }
}

// Implementing &Vector2D<T> * T -> Vector2D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for &'a Vector2D<T> {
    type Output = Vector2D<T>;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        Vector2D { x: self.x * scalar,
                   y: self.y * scalar }
    }
}


//////////////////////////////////// Right Side Scalar Division ////////////////////////////////////

// Implementing Vector2D<T> / T -> Vector2D<T> type multiplication
impl<T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for Vector2D<T> {
    type Output = Self;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        Self { x: self.x / scalar,
               y: self.y / scalar }
    }
}

// Implementing &Vector2D<T> / T -> Vector2D<T> type multiplication
impl<'a, T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for &'a Vector2D<T> {
    type Output = Vector2D<T>;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        Vector2D { x: self.x / scalar,
                   y: self.y / scalar }
    }
}

// ===================================  VectorOperations impl =================================== //

impl<T: DataTypeTraits> VectorOperations<T> for Vector2D<T> {

    #[inline(always)]
    fn inner_product(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
}


/******************************************** 3D VECTOR *******************************************/


// Implementing fmt::Display, which uses the {} print marker
impl<T: DataTypeTraits> std::fmt::Display for Vector3D<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write the first and second element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

// Implementing fmt::Debug, which uses the {:?} print marker
impl<T: DataTypeTraits>  std::fmt::Debug for Vector3D<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector3D")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.y)
            .finish()
    }
}

// Implementing Vector3D<T> initialization through <T>::new()
impl<T: DataTypeTraits> Vector3D<T>
{
    pub fn new() -> Self {
        Vector3D {x: T::default(),
                  y: T::default(),
                  z: T::default()}
    }
}
// Implementing Vector3D<T> initialization through <T>::default()
impl<T: DataTypeTraits> Default for Vector3D<T> {
    fn default() -> Self {
        Vector3D { x: T::zero(),
                   y: T::zero(),
                   z: T::zero() }
    }
}

// Implementing Vector3D<T> with zeros initialization through <T>::zeros()
impl<T: DataTypeTraits> Vector3D<T>
{
    pub fn zeros() -> Self {
        Vector3D { x: T::zero(),
                   y: T::zero(),
                   z: T::zero()}
    }
}

// Implementing Vector3D<T> with ones initialization through <T>::ones()
impl<T: DataTypeTraits> Vector3D<T>
{
    pub fn ones() -> Self {
        Vector3D { x: T::one(),
                   y: T::one(),
                   z: T::one()}
    }
}


// Implementing Vector3D<T> with random uniform in [low,high[ initialization through <T>::random_uniform()
impl<T: DataTypeTraits> Vector3D<T>
{
    pub fn random_uniform(lower_bound: T, upper_bound: T) -> Vector3D<T> {
        if upper_bound <= lower_bound {
            panic!("Upper bound cannot be less than or equal to lower bound");
        }
        let distribution = Uniform::try_from(lower_bound..upper_bound).unwrap();
        let mut rng = rand::thread_rng();
        Vector3D { x: distribution.sample(&mut rng),
                   y: distribution.sample(&mut rng),
                   z: distribution.sample(&mut rng)}
    }
}

// Implementing Vector3D<T> with random uniform in [low,high[ initialization through <T>::fast_random_uniform()
impl<T: DataTypeTraits> Vector3D<T>
{
    pub fn fast_random_uniform(lower_bound: T, upper_bound: T) -> Vector3D<T> {
        if upper_bound <= lower_bound {
            panic!("Upper bound cannot be less than or equal to lower bound");
        }
        let rng = fastrand::Rng::new();
        if std::mem::size_of::<T>() == std::mem::size_of::<f32>() {
            Vector3D { x: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound),
                       y: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound),
                       z: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound)}
        }
        else {
            Vector3D { x: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound),
                       y: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound),
                       z: lower_bound + T::from(rng.f32()).unwrap() * (upper_bound - lower_bound)}
        }
    }
}



//++++++++++++++++++++++++++++++++++++++++ Addition ++++++++++++++++++++++++++++++++++++++++++++ //
// Implementing Vector3D<T> + Vector3D<T> -> Vector3D<T> type addition
impl<T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<Vector3D<T>> for Vector3D<T> {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self { x: self.x + other.x,
               y: self.y + other.y,
               z: self.z + other.z}
    }
}
// Implementing &Vector3D<T> + &Vector3D<T> -> Vector3D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn add(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x + other.x,
                   y: self.y + other.y,
                   z: self.z + other.z}
    }
}
// Implementing Vector3D<T> + &Vector3D<T> -> Vector3D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a Vector3D<T>> for Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn add(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x + other.x,
                   y: self.y + other.y,
                   z: self.z + other.z}
    }
}
// Implementing &Vector3D<T> + Vector3D<T> -> Vector3D<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn add(self, other: Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x + other.x,
                   y: self.y + other.y,
                   z: self.z + other.z}
    }
}


//----------------------------------------- Subtraction ----------------------------------------- //

// Implementing Vector3D<T> - Vector3D<T> -> Vector3D<T> type subtraction
impl<T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<Vector3D<T>> for Vector3D<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self { x: self.x - other.x,
               y: self.y - other.y,
               z: self.z - other.z}
    }
}
// Implementing &Vector3D<T> - &Vector3D<T> -> Vector3D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn sub(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x - other.x,
                   y: self.y - other.y,
                   z: self.z - other.z}
    }
}
// Implementing Vector3D<T> - &Vector3D<T> -> Vector3D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a Vector3D<T>> for Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn sub(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x - other.x,
                   y: self.y - other.y,
                   z: self.z - other.z}
    }
}
// Implementing &Vector3D<T> - Vector3D<T> -> Vector3D<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn sub(self, other: Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x - other.x,
                   y: self.y - other.y,
                   z: self.z - other.z}
    }
}


//********************************** Entry-wise multiplication ***********************************//

// Implementing Vector3D<T> * Vector3D<T> -> Vector3D<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<Vector3D<T>> for Vector3D<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self { x: self.x * other.x,
               y: self.y * other.y,
               z: self.z * other.z}
    }
}
// Implementing &Vector3D<T> * &Vector3D<T> -> Vector3D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn mul(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x * other.x,
                   y: self.y * other.y,
                   z: self.z * other.z}
    }
}
// Implementing Vector3D<T> * &Vector3D<T> -> Vector3D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a Vector3D<T>> for Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn mul(self, other: &Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x * other.x,
                   y: self.y * other.y,
                   z: self.z * other.z}
    }
}
// Implementing &Vector3D<T> * Vector3D<T> -> Vector3D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<Vector3D<T>> for &'a Vector3D<T>
{
    type Output = Vector3D<T>;
    #[inline(always)]
    fn mul(self, other: Vector3D<T>) -> Self::Output {
        Vector3D { x: self.x * other.x,
                   y: self.y * other.y,
                   z: self.z * other.z}
    }
}


//******************************* Right Side Scalar multiplication *******************************//

// Implementing Vector3D<T> * T -> Vector3D<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for Vector3D<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        Self { x: self.x * scalar,
               y: self.y * scalar,
               z: self.z * scalar}
    }
}

// Implementing &Vector3D<T> * T -> Vector3D<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for &'a Vector3D<T> {
    type Output = Vector3D<T>;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        Vector3D { x: self.x * scalar,
                   y: self.y * scalar,
                   z: self.z * scalar}
    }
}


//////////////////////////////////// Right Side Scalar Division ////////////////////////////////////

// Implementing Vector3D<T> / T -> Vector3D<T> type multiplication
impl<T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for Vector3D<T> {
    type Output = Self;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        Self { x: self.x / scalar,
               y: self.y / scalar,
               z: self.z / scalar}
    }
}

// Implementing &Vector3D<T> / T -> Vector3D<T> type multiplication
impl<'a, T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for &'a Vector3D<T> {
    type Output = Vector3D<T>;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        Vector3D { x: self.x / scalar,
                   y: self.y / scalar,
                   z: self.z / scalar}
    }
}

// ===================================  VectorOperations impl =================================== //

impl<T: DataTypeTraits> VectorOperations<T> for Vector3D<T> {

    #[inline(always)]
    fn inner_product(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// =================================== Special Vector3D impls =================================== //

// Implementing Vector3D<T>.cross_product(Vector3D<T>)
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> Vector3D<T>{
    #[inline(always)]
    fn cross_product(&self, other: &Self) -> Vector3D<T> {
        Self { x: self.y * other.z - self.z * other.y,
               y: self.z * other.x - self.x * other.z,
               z: self.x * other.y - self.y * other.x}
    }
}
