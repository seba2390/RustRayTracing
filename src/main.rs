use std::io::Write;

use ray_lib::Ray3D;

use vector_lib::Vector2D;
use vector_lib::Vector3D;
use vector_lib::VectorOperations;
use vector_lib::DataTypeTraits;

use color_lib::RGBColor;
use scene_lib::Scene;
use sphere_lib::Sphere;


// see: https://raytracing.github.io/books/RayTracingInOneWeekend.html



fn main() -> std::io::Result<()> {


////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// TEST OF .PPM FORMAT /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////// CREATING GRADIENT //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////



    //====== Image ======//
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 1000;
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
            let color: RGBColor<f64> =utilities_lib::ray_color(&ray);
            utilities_lib::write_color(&file,color)?;
        }
    }
    match utilities_lib::convert_to_png(file_name) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error: {}", e),
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////// CREATING GRADIENT W. RED BALL ////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////



    //====== Render ======//
    // Creating ball
    let sphere: Sphere<f64> = Sphere::new(Vector3D{x:0.0_f64, y:0.0_f64, z: -1.0_f64}, 0.5_f64);
    // Creating new file
    let file_name = "gradient_w_ball.ppm";
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
            let color: RGBColor<f64> = utilities_lib::ray_color_2(&ray, &sphere);
            utilities_lib::write_color(&file,color)?;
        }
    }
    match utilities_lib::convert_to_png(file_name) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error: {}", e),
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// CREATING GRADIENT W. GRADIENT BALL //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

    // Creating new file
    let file_name = "gradient_w_gradient_ball.ppm";
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
            let color: RGBColor<f64> = utilities_lib::ray_color_3(&ray, &sphere);
            utilities_lib::write_color(&file,color)?;
        }
    }
    match utilities_lib::convert_to_png(file_name) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error: {}", e),
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////// CREATING GRADIENT W. GRADIENT BALL ON GROUND /////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

    // Creating ball
    let sphere: Sphere<f64> = Sphere::new(Vector3D{x:0.0_f64, y:0.0_f64, z: -1.0_f64}, 0.5_f64);
    // Creating large ball to emulate ground
    let ground: Sphere<f64> = Sphere::new(Vector3D{x:0.0_f64, y:-100.5_f64, z: -1.0_f64}, 100.0_f64);

    // Creating scene
    let mut scene = Scene::default();
    scene.add(Box::new(sphere));
    scene.add(Box::new(ground));

    // Creating new file
    let file_name = "gradient_w_gradient_ball_on_ground.ppm";
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
                direction: &lower_left_corner + &horizontal * u + &vertical * v};
            let color: RGBColor<f64> = utilities_lib::ray_color_4(&ray, &mut scene);
            utilities_lib::write_color(&file,color)?;
        }
    }
    match utilities_lib::convert_to_png(file_name) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}