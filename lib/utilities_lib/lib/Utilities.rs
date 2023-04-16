use std::io::Write;

use hittable_traits::{HitRecord, Hittable};

use color_lib::RGBColor;

use vector_lib::Vector3D;
use vector_lib::DataTypeTraits;
use vector_lib::VectorOperations;

use sphere_lib::Sphere;

use ray_lib::Ray3D;

use scene_lib::Scene;

// Constants
const F32_INFINITY: f32 = f32::INFINITY;
const F32_PI: f32 = std::f32::consts::PI;



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
    return RGBColor{R: T::one(), G:  T::one(), B:  T::one()} * ( T::one() - t) +
        RGBColor{R: T::from(0.5).unwrap(), G: T::from(0.7).unwrap(), B:  T::one()} * t
}


#[inline(always)]
pub fn ray_color_2<T: DataTypeTraits>(ray: &Ray3D<T>, sphere: &Sphere<T>) -> RGBColor<T> {
    if sphere.is_hit(ray)
    {
        return RGBColor{R: T::one(), G: T::zero(), B: T::zero()}
    }
    else {
        return ray_color(ray)
    }
}

#[inline(always)]
pub fn ray_color_3<T: DataTypeTraits>(ray: &Ray3D<T>, sphere: &Sphere<T>) -> RGBColor<T> {
    let t = sphere.is_hit_at(ray);
    if t > T::zero()
    {
        // Unnormalized normal vector of sphere at point of intersection w. ray
        #[allow(non_snake_case)]
            let N: Vector3D<T> = (ray.at(t)-sphere.center).unit_vector();
        RGBColor{R: N.x + T::one(), G: N.y + T::one(),B: N.z + T::one()} * T::from(0.5).unwrap()
    }
    else {
        ray_color(ray)
    }
}

#[inline(always)]
pub fn ray_color_4<T: DataTypeTraits>(ray: &Ray3D<T>, scene: &mut Scene<T>) -> RGBColor<T> {
    let mut hit_record = HitRecord::default();
    if scene.hit(ray, T::zero(), T::from(F32_INFINITY).unwrap(), &mut hit_record) {
        #[allow(non_snake_case)]
            let N = hit_record.get_normal_vector();
        return (RGBColor{R: N.x, G: N.y, B: N.z} + RGBColor{R: T::one(), G: T::one(), B: T::one()})
            * T::from(0.5).unwrap()
    }
    // Unnormalized normal vector of sphere at point of intersection w. ray
    let unit_direction = ray.direction.unit_vector();
    let t = T::from(0.5).unwrap() * (unit_direction.y + T::one());
    return RGBColor{R: T::one(), G:  T::one(), B:  T::one()} * ( T::one() - t) +
        RGBColor{R: T::from(0.5).unwrap(), G: T::from(0.7).unwrap(), B:  T::one()} * t

}



/// Generates a random number of type `T` within the given range. If no range is specified, the random number
/// will be generated within the maximum value of type `T`. The function returns a random number of type `T`.
///
/// # Arguments
///
/// * `min_value`: An optional parameter of type `Option<T>`. If specified, represents the minimum value of the range.
/// * `max_value`: An optional parameter of type `Option<T>`. If specified, represents the maximum value of the range.
///
/// # Panics
///
/// The function will panic if `T` is not of type `f32` or `f64`.
///
/// # Examples
///
/// Generating a random float between 0.0 and 1.0:
///
/// ```
/// let rand_num = generate_random_number::<f32>(Some(0.0), Some(1.0));
/// ```
///
/// Generating a random float with no specified range (random float within maximum value of `f64`):
///
/// ```
/// let rand_num = generate_random_number::<f64>(None, None);
/// ```
#[inline(always)]
pub fn generate_random_number<T: DataTypeTraits>(min_value: Option<T>, max_value: Option<T>) -> T {
    let mut rng = fastrand::Rng::new();
    if std::mem::size_of::<T>() == std::mem::size_of::<f32>() {
        let random_float = T::from(rng.f32()).unwrap();
        match (min_value, max_value) {
            (Some(min), Some(max)) => return min + random_float * (max - min),
            (Some(min), None) => return min + random_float * (T::max_value() - min),
            (None, Some(max)) => return random_float * max,
            (None, None) => return random_float * T::max_value(),
        }
    }
    if std::mem::size_of::<T>() == std::mem::size_of::<f64>() {
        let random_float = T::from(rng.f64()).unwrap();
        match (min_value, max_value) {
            (Some(min), Some(max)) => return min + random_float * (max - min),
            (Some(min), None) => return min + random_float * (T::max_value() - min),
            (None, Some(max)) => return random_float * max,
            (None, None) => return random_float * T::max_value(),
        }
    }
    panic!("Invalid 'T' type - expected f32 or f64.");
}


pub fn write_color<T: DataTypeTraits>(mut file: &std::fs::File, color: RGBColor<T>) -> std::io::Result<()>
{
    let factor: f64 = 255.999;
    let integer_red   = (factor * color.R.to_f64().unwrap()) as i32;
    let integer_green = (factor * color.G.to_f64().unwrap()) as i32;
    let integer_blue  = (factor * color.B.to_f64().unwrap()) as i32;
    writeln!(file, "{} {} {}", integer_red, integer_green, integer_blue)?;
    Ok(())
}

pub fn write_color_2<T: DataTypeTraits>(mut file: &std::fs::File, color: RGBColor<T>, samples_per_pixel: u32) -> std::io::Result<()>
{
    let factor: f64 = 256.0;
    let integer_red   = (factor * clamp(color.R.to_f64().unwrap() / samples_per_pixel as f64,0.0, 0.999)) as i32;
    let integer_green = (factor * clamp(color.G.to_f64().unwrap() / samples_per_pixel as f64,0.0, 0.999)) as i32;
    let integer_blue  = (factor * clamp(color.B.to_f64().unwrap() / samples_per_pixel as f64,0.0, 0.999)) as i32;
    writeln!(file, "{} {} {}", integer_red, integer_green, integer_blue)?;
    Ok(())
}

#[inline(always)]
pub fn degrees_to_radians<T: DataTypeTraits>(degrees: T) -> T {
    degrees * T::from(F32_PI).unwrap() / T::from(180.0).unwrap()
}


pub fn convert_to_png(file_name: &str) -> Result<(), String> {
    let output_file_name = format!("{}.png", &file_name[..file_name.len() - 4]);
    let command = format!("pnmtopng {} > {}", file_name, output_file_name);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| format!("failed to execute process 1: {}", e))?;

    if !output.status.success() {
        return Err(format!("process exited with code {}", output.status));
    }
    let command = format!("rm {}", file_name);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| format!("failed to execute process 2: {}", e))?;

    if !output.status.success() {
        return Err(format!("process exited with code {}", output.status));
    }

    Ok(())
}

#[inline(always)]
pub fn clamp<T: DataTypeTraits>(x: T, min: T, max: T) -> T
{
    if x < min { min} else if x > max { max} else { x }
}

