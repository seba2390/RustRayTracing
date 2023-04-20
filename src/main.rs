use std::time::Instant;

use std::io::Write;
use camera_lib::Camera;

use ray_lib::Ray3D;

use vector_lib::Vector3D;

use color_lib::RGBColor;
use scene_lib::Scene;
use sphere_lib::Sphere;




// see: https://raytracing.github.io/books/RayTracingInOneWeekend.html



fn main() -> std::io::Result<()> {



////////////////////////////////////////////////////////////////////////////////////////////////////
//// ANTI-ALIASING GRADIENT W. GRADIENT BALL ON GROUND + DIFFUSION + TRUE LAMBERTIAN REFLECTION ////
////////////////////////////////////////////////////////////////////////////////////////////////////

    // Defining camera for scene
    const ORIGIN_4: Vector3D<f64> = Vector3D{ x: 0.0_f64, y: 0.0_f64, z: 0.0_f64};
    const VIEWPORT_HEIGHT_4: f64 = 2.0;
    const FOCAL_LENGTH_4: f64 = 1.0;
    const ASPECT_RATIO_4: f64 = 16.0 / 9.0;
    const IMG_WIDTH_4: i32 = 600;
    const IMG_HEIGHT_4: i32 = (IMG_WIDTH_4 as f64 / ASPECT_RATIO_4) as i32;
    const SAMPLES_PER_PIXEL_4: i32 = 300;
    const MAX_DEPTH_4: i32 = 50;

    let camera: Camera<f64> = Camera::new(ASPECT_RATIO_4, VIEWPORT_HEIGHT_4,
                                          FOCAL_LENGTH_4, ORIGIN_4);
    // Creating balls
    let center_1 = Vector3D{x:0.0_f64, y:0.0_f64, z: -1.0_f64};
    let radius_1 = 0.5_f64;
    let sphere_1: Sphere<f64> = Sphere::new(center_1, radius_1);

    // Creating large ball to emulate ground
    let ground: Sphere<f64> = Sphere::new(Vector3D{x:0.0_f64, y:-100.5_f64, z: -1.0_f64}, 100.0_f64);

    // Creating scene
    let mut scene = Scene::default();
    scene.add(Box::new(sphere_1));
    scene.add(Box::new(ground));


    // Creating new file
    let file_name = "renders/lambertian_diffusion_antialiasing_gradient_w_gradient_ball_on_ground.ppm";
    let mut file = std::fs::OpenOptions::new().create(true).write(true).open(file_name)?;
    // Header info for .ppm file
    write!(file, "P3\n{} {}\n255\n", IMG_WIDTH_4, IMG_HEIGHT_4)?;
    // Progress bar
    let bar = indicatif::ProgressBar::new(IMG_HEIGHT_4 as u64);
    let start = Instant::now();
    for j in (0..IMG_HEIGHT_4).rev() {
        bar.inc(1);
        for i in 0..IMG_WIDTH_4 {
            let mut color_sum: RGBColor<f64> = RGBColor{R: 0.0_f64, G: 0.0_f64, B: 0.0_f64};
            for _sample in 0..SAMPLES_PER_PIXEL_4
            {
                let u: f64 = (f64::from(i) + utilities_lib::generate_random_uniform::<f64>(0.0_f64,1.0_f64))/ f64::from(IMG_WIDTH_4-1);
                let v: f64 = (f64::from(j) + utilities_lib::generate_random_uniform::<f64>(0.0_f64,1.0_f64))/ f64::from(IMG_HEIGHT_4-1);
                let ray: Ray3D<f64> = camera.get_ray(u,v);
                color_sum = color_sum + utilities_lib::ray_color(&ray, &mut scene, MAX_DEPTH_4);
            }
            utilities_lib::write_color(&file, color_sum, SAMPLES_PER_PIXEL_4 as u32)?;
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    match utilities_lib::convert_to_png(file_name) {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}