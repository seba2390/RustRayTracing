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
std::clone::Clone{
    // we'd usually add more functions in this block,
    // but in this case we don't need any more.
}

// 2. Implement it
impl<T> DataTypeTraits for T
    where T: num_traits::Float + std::fmt::Display + std::fmt::Debug +
    std::marker::Copy + std::default::Default + num_traits::Zero +
    num_traits::One + rand::distributions::uniform::SampleUniform +
    std::clone::Clone{
    // Nothing to implement, since T already supports the other traits.
    // It has the functions it needs already
}


////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// STRUCT DEFINITIONS //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(non_snake_case)]
#[derive(PartialEq, Clone, Copy)]
pub struct RGBColor<T: DataTypeTraits> {
    pub R: T,
    pub G: T,
    pub B: T
}




// Implementing fmt::Display, which uses the {} print marker
impl<T: DataTypeTraits> std::fmt::Display for RGBColor<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write the first and second element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}, {}", self.R, self.G, self.B)
    }
}

// Implementing fmt::Debug, which uses the {:?} print marker
impl<T: DataTypeTraits>  std::fmt::Debug for RGBColor<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RGBColor")
            .field("R", &self.R)
            .field("G", &self.G)
            .field("B", &self.G)
            .finish()
    }
}

// Implementing RGBColor<T> initialization through <T>::new()
impl<T: DataTypeTraits> RGBColor<T>
{
    pub fn new() -> Self {
        RGBColor {R: T::default(),
            G: T::default(),
            B: T::default()}
    }
}

// Implementing RGBColor<T> with zeros initialization through <T>::zeros()
impl<T: DataTypeTraits> RGBColor<T>
{
    pub fn zeros() -> Self {
        RGBColor { R: T::zero(),
            G: T::zero(),
            B: T::zero()}
    }
}

// Implementing RGBColor<T> with ones initialization through <T>::ones()
impl<T: DataTypeTraits> RGBColor<T>
{
    pub fn ones() -> Self {
        RGBColor { R: T::one(),
            G: T::one(),
            B: T::one()}
    }
}


// Implementing RGBColor<T> with random uniform in [low,high[ initialization through <T>::random_uniform()
impl<T: DataTypeTraits> RGBColor<T>
{
    pub fn random_uniform(lower_bound: T, upper_bound: T) -> RGBColor<T> {
        if upper_bound <= lower_bound {
            panic!("Upper bound cannot be less than or equal to lower bound");
        }
        let distribution = Uniform::try_from(lower_bound..upper_bound).unwrap();
        let mut rng = rand::thread_rng();
        RGBColor { R: distribution.sample(&mut rng),
            G: distribution.sample(&mut rng),
            B: distribution.sample(&mut rng)}
    }
}
//++++++++++++++++++++++++++++++++++++++++ Addition ++++++++++++++++++++++++++++++++++++++++++++ //
// Implementing RGBColor<T> + RGBColor<T> -> RGBColor<T> type addition
impl<T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<RGBColor<T>> for RGBColor<T> {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self { R: self.R + other.R,
            G: self.G + other.G,
            B: self.B + other.B}
    }
}
// Implementing &RGBColor<T> + &RGBColor<T> -> RGBColor<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn add(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R + other.R,
            G: self.G + other.G,
            B: self.B + other.B}
    }
}
// Implementing RGBColor<T> + &RGBColor<T> -> RGBColor<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<&'a RGBColor<T>> for RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn add(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R + other.R,
            G: self.G + other.G,
            B: self.B + other.B}
    }
}
// Implementing &RGBColor<T> + RGBColor<T> -> RGBColor<T> type addition
impl<'a, T: std::ops::Add<Output = T> + DataTypeTraits> std::ops::Add<RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn add(self, other: RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R + other.R,
            G: self.G + other.G,
            B: self.B + other.B}
    }
}


//----------------------------------------- Subtraction ----------------------------------------- //

// Implementing RGBColor<T> - RGBColor<T> -> RGBColor<T> type subtraction
impl<T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<RGBColor<T>> for RGBColor<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self { R: self.R - other.R,
            G: self.G - other.G,
            B: self.B - other.B}
    }
}
// Implementing &RGBColor<T> - &RGBColor<T> -> RGBColor<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn sub(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R - other.R,
            G: self.G - other.G,
            B: self.B - other.B}
    }
}
// Implementing RGBColor<T> - &RGBColor<T> -> RGBColor<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<&'a RGBColor<T>> for RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn sub(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R - other.R,
            G: self.G - other.G,
            B: self.B - other.B}
    }
}
// Implementing &RGBColor<T> - RGBColor<T> -> RGBColor<T> type subtraction
impl<'a, T: std::ops::Sub<Output = T> + DataTypeTraits> std::ops::Sub<RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn sub(self, other: RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R - other.R,
            G: self.G - other.G,
            B: self.B - other.B}
    }
}


//********************************** Entry-wise multiplication ***********************************//

// Implementing RGBColor<T> * RGBColor<T> -> RGBColor<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<RGBColor<T>> for RGBColor<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self { R: self.R * other.R,
            G: self.G * other.G,
            B: self.B * other.B}
    }
}
// Implementing &RGBColor<T> * &RGBColor<T> -> RGBColor<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn mul(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R * other.R,
            G: self.G * other.G,
            B: self.B * other.B}
    }
}
// Implementing RGBColor<T> * &RGBColor<T> -> RGBColor<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<&'a RGBColor<T>> for RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn mul(self, other: &RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R * other.R,
            G: self.G * other.G,
            B: self.B * other.B}
    }
}
// Implementing &RGBColor<T> * RGBColor<T> -> RGBColor<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<RGBColor<T>> for &'a RGBColor<T>
{
    type Output = RGBColor<T>;
    #[inline(always)]
    fn mul(self, other: RGBColor<T>) -> Self::Output {
        RGBColor { R: self.R * other.R,
            G: self.G * other.G,
            B: self.B * other.B}
    }
}


//******************************* Right Side Scalar multiplication *******************************//

// Implementing RGBColor<T> * T -> RGBColor<T> type multiplication
impl<T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for RGBColor<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        Self { R: self.R * scalar,
            G: self.G * scalar,
            B: self.B * scalar}
    }
}

// Implementing &RGBColor<T> * T -> RGBColor<T> type multiplication
impl<'a, T: std::ops::Mul<Output = T> + DataTypeTraits> std::ops::Mul<T> for &'a RGBColor<T> {
    type Output = RGBColor<T>;
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        RGBColor { R: self.R * scalar,
            G: self.G * scalar,
            B: self.B * scalar}
    }
}


//////////////////////////////////// Right Side Scalar Division ////////////////////////////////////

// Implementing RGBColor<T> / T -> RGBColor<T> type multiplication
impl<T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for RGBColor<T> {
    type Output = Self;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        Self { R: self.R / scalar,
            G: self.G / scalar,
            B: self.B / scalar}
    }
}

// Implementing &RGBColor<T> / T -> RGBColor<T> type multiplication
impl<'a, T: std::ops::Div<Output = T> + DataTypeTraits> std::ops::Div<T> for &'a RGBColor<T> {
    type Output = RGBColor<T>;
    #[inline(always)]
    fn div(self, scalar: T) -> Self::Output {
        RGBColor { R: self.R / scalar,
            G: self.G / scalar,
            B: self.B / scalar}
    }
}
