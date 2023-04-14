use std::io::Write;
use ray_lib::Ray3D;
use vector_lib::Vector2D;
use vector_lib::Vector3D;
use vector_lib::VectorOperations;
use vector_lib::DataTypeTraits;
use color_lib::RGBColor;

// see: https://raytracing.github.io/books/RayTracingInOneWeekend.html


fn write_color<T: DataTypeTraits>(mut file: &std::fs::File, color: RGBColor<T>) -> std::io::Result<()>
{
    let factor: f64 = 255.999;
    let integer_red   = (factor * color.R.to_f64().unwrap()) as i32;
    let integer_green = (factor * color.G.to_f64().unwrap()) as i32;
    let integer_blue  = (factor * color.B.to_f64().unwrap()) as i32;
    writeln!(file, "{} {} {}", integer_red, integer_green, integer_blue)?;
    Ok(())
}



fn main() -> std::io::Result<()> {
    /*
    // See: https://en.wikipedia.org/wiki/Netpbm#PPM_example for details regarding .ppm format
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    
    // Creating new file
    let file_name = "test.ppm";
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open(file_name)?;

    // Render
    write!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    // Defining progressbar to visualize progress
    let bar = indicatif::ProgressBar::new(IMAGE_HEIGHT as u64);

    for j in 0..IMAGE_HEIGHT {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            let float_r = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let float_g = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let float_b = 0.25;

            let integer_r = (255.999 * float_r) as i32;
            let integer_g = (255.999 * float_g) as i32;
            let integer_b = (255.999 * float_b) as i32;

            writeln!(file, "{} {} {}", integer_r, integer_g, integer_b)?;
        }
    }

    Ok(())

     */

    //====== Image ======//
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 600;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;

    //====== Camera ======//
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Vector3D { x: 0.0_f64, y: 0.0_f64, z: 0.0_f64 };

    let horizontal = Vector3D { x: viewport_width, y: 0.0_f64, z: 0.0_f64 };
    let vertical = Vector3D { x: 0.0_f64, y: viewport_height, z: 0.0_f64 };
    let lower_left_corner = &origin - &horizontal / 2.0_f64 -
        &vertical / 2.0_f64 - Vector3D { x: 0.0_f64, y: 0.0_f64, z: focal_length };

    //====== Render ======//
    // Creating new file
    let file_name = "gradient.ppm";
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open(file_name)?;
    // Header info for .ppm file
    write!(file, "P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT)?;
    // Progress bar
    let bar = indicatif::ProgressBar::new(IMG_HEIGHT as u64);
    for j in (0..IMG_HEIGHT).rev() {
        bar.inc(1);
        for i in 0..IMG_WIDTH {
            let u = f64::from(i) / f64::from(IMG_WIDTH-1);
            let v = f64::from(j) / f64::from(IMG_HEIGHT-1);

            let ray: Ray3D<f64> = Ray3D{origin: origin.clone(),
                                        direction: &lower_left_corner + &horizontal * u +
                                                   &vertical * v - &origin};
            let color: RGBColor<f64> = math_lib::ray_color(&ray);
            write_color(&file,color)?;
        }
    }
    Ok(())
}