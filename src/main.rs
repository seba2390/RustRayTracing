use std::io::Write;
use vector_lib::Vector2D;
use vector_lib::Vector3D;
use vector_lib::VectorOperations;

// see: https://raytracing.github.io/books/RayTracingInOneWeekend.html

fn main() -> std::io::Result<()> {

    /*
    println!(" ============= 2D VECTOR =================");

    // Initialization
    let v1 = Vector2D::<f64>::new();
    let v2 = Vector2D::<f64>::ones();
    let v3 = Vector2D::<f64>::zeros();
    let v4 = Vector2D::<f64>::random_uniform(-1.0,1.0);

    println!("new() : {}", v1);
    println!("ones() : {}", v2);
    println!("zeros() : {}", v3);
    println!("random_uniform() : {}", v4);

    println!("=======================");

    // Addition
    let v1 = Vector2D{x: 0.0, y: 1.3};
    let v2 = Vector2D{x: 1.5, y: 0.0};
    let v3 = Vector2D{x: 0.0, y: 1.3};
    let v4 = Vector2D{x: 1.5, y: 0.0};
    let v5 = Vector2D{x: 0.0, y: 1.3};
    let v6 = Vector2D{x: 1.5, y: 0.0};

    println!("v1=v3=v5: {}", v1);
    println!("v2=v4=v6: {}", v2);
    println!("&v1 + &v2: {}", &v1+&v2);
    println!("v1 + v2: {}", v1+v2);
    println!("v3 + &v4: {}", v3+&v4);
    println!("&v3 + v4: {}", &v5+v6);

    println!("=======================");

    // Subtraction
    let v1 = Vector2D{x: 0.0, y: 1.3};
    let v2 = Vector2D{x: 1.5, y: 0.0};
    let v3 = Vector2D{x: 0.0, y: 1.3};
    let v4 = Vector2D{x: 1.5, y: 0.0};
    let v5 = Vector2D{x: 0.0, y: 1.3};
    let v6 = Vector2D{x: 1.5, y: 0.0};

    println!("&v1 - &v2: {}", &v1-&v2);
    println!("v1 - v2: {}", v1-v2);
    println!("v3 - &v4: {}", v3-&v4);
    println!("&v3 - v4: {}", &v5-v6);

    println!("=======================");

    // Entry-wise Multiplication
    let v1 = Vector2D{x: 0.1, y: 1.3};
    let v2 = Vector2D{x: 1.5, y: 1.0};
    let v3 = Vector2D{x: 0.1, y: 1.3};
    let v4 = Vector2D{x: 1.5, y: 1.0};
    let v5 = Vector2D{x: 0.1, y: 1.3};
    let v6 = Vector2D{x: 1.5, y: 1.0};

    println!("v1=v3=v5: {}", v1);
    println!("v2=v4=v6: {}", v2);
    println!("&v1 * &v2: {}", &v1 * &v2);
    println!("v1 * v2: {}", v1 * v2);
    println!("v3 * &v4: {}", v3 * &v4);
    println!("&v3 * v4: {}", &v5 * v6);

    println!("=======================");

    // Components
    let v1 = Vector2D{x: 0.1, y: 1.3};

    println!("v1: {}", v1);
    println!("v1.x: {}", v1.x);
    println!("v1.y: {}", v1.y);
    println!("=======================");


    // Scalar Multiplication
    let v1 = Vector2D{x: 0.1, y: 1.3};
    let c: f64 = 2.3;

    println!("c: {}", c);
    println!("v1: {}", v1);
    println!("&v1 * c: {}", &v1 * c);
    println!(" v1 * c: {}",   v1 * c);

    println!("=======================");

    // Vector operations
    let v1 = Vector2D{x: 0.1, y: 1.3};
    let v2 = Vector2D{x: 1.5, y: 1.0};

    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("<v1,v2> : {}", v1.inner_product(&v2));
    println!("<v2,v1> : {}", v2.inner_product(&v1));
    println!("<v1,v1> : {}", v1.inner_product(&v1));
    println!("<v2,v2> : {}", v2.inner_product(&v2));
    println!("||v1|| : {}", v1.norm());
    println!("||v2|| : {}", v2.norm());
    println!("=======================");

    // Normalization
    let mut v1 = Vector2D{x: 0.1, y: 1.3};

    println!("Initial    v1: {} w. norm: {}", v1,v1.norm());
    v1.normalize();
    println!("Normalized v1: {} w. norm: {}", v1, v1.norm());

    println!("=======================");

    // Projection
    let v1 = Vector2D{x: 0.1, y: 0.87};
    let v2 = Vector2D{x: 0.0, y: 1.3};
    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v1 projected onto v2: {}", v1.project(&v2));


    println!(" ============= 3D VECTOR =================");

    // Initialization
    let v1 = Vector3D::<f64>::new();
    let v2 = Vector3D::<f64>::ones();
    let v3 = Vector3D::<f64>::zeros();
    let v4 = Vector3D::<f64>::random_uniform(-1.0,1.0);

    println!("new() : {}", v1);
    println!("ones() : {}", v2);
    println!("zeros() : {}", v3);
    println!("random_uniform() : {}", v4);

    println!("=======================");

    // Addition
    let v1 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v2 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v3 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v4 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v5 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v6 = Vector3D{x: 1.5, y: 0.0, z: 0.0};

    println!("v1=v3=v5: {}", v1);
    println!("v2=v4=v6: {}", v2);
    println!("&v1 + &v2: {}", &v1+&v2);
    println!("v1 + v2: {}", v1+v2);
    println!("v3 + &v4: {}", v3+&v4);
    println!("&v3 + v4: {}", &v5+v6);

    println!("=======================");

    // Subtraction
    let v1 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v2 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v3 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v4 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v5 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v6 = Vector3D{x: 1.5, y: 0.0, z: 0.0};

    println!("&v1 - &v2: {}", &v1-&v2);
    println!("v1 - v2: {}", v1-v2);
    println!("v3 - &v4: {}", v3-&v4);
    println!("&v3 - v4: {}", &v5-v6);

    println!("=======================");

    // Entry-wise Multiplication
    let v1 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v2 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v3 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v4 = Vector3D{x: 1.5, y: 0.0, z: 0.0};
    let v5 = Vector3D{x: 0.0, y: 1.3, z: 0.0};
    let v6 = Vector3D{x: 1.5, y: 0.0, z: 0.0};

    println!("v1=v3=v5: {}", v1);
    println!("v2=v4=v6: {}", v2);
    println!("&v1 * &v2: {}", &v1 * &v2);
    println!("v1 * v2: {}", v1 * v2);
    println!("v3 * &v4: {}", v3 * &v4);
    println!("&v3 * v4: {}", &v5 * v6);

    println!("=======================");

    // Components
    let v1 = Vector3D{x: 0.1, y: 1.3, z: 0.1};

    println!("v1: {}", v1);
    println!("v1.x: {}", v1.x);
    println!("v1.y: {}", v1.y);
    println!("=======================");


    // Scalar Multiplication
    let v1 = Vector3D{x: 0.1, y: 1.3, z: 0.1};
    let c: f64 = 2.3;

    println!("c: {}", c);
    println!("v1: {}", v1);
    println!("&v1 * c: {}", &v1 * c);
    println!(" v1 * c: {}",   v1 * c);

    println!("=======================");

    // Vector operations
    let v1 = Vector3D{x: 0.1, y: 1.3, z: 0.1};
    let v2 = Vector3D{x: 1.5, y: 1.0, z: 0.6};

    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("<v1,v2> : {}", v1.inner_product(&v2));
    println!("<v2,v1> : {}", v2.inner_product(&v1));
    println!("<v1,v1> : {}", v1.inner_product(&v1));
    println!("<v2,v2> : {}", v2.inner_product(&v2));
    println!("||v1|| : {}", v1.norm());
    println!("||v2|| : {}", v2.norm());
    println!("=======================");

    // Normalization
    let mut v1 = Vector3D{x: 0.1, y: 1.3, z: 0.1};

    println!("Initial    v1: {} w. norm: {}", v1,v1.norm());
    v1.normalize();
    println!("Normalized v1: {} w. norm: {}", v1, v1.norm());

    println!("=======================");

    // Projection
    let v1 = Vector3D{x: 0.1, y: 0.87, z: 9.1};
    let v2 = Vector3D{x: 0.0, y: 1.3,  z: 3.1};
    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v1 projected onto v2: {}", v1.project(&v2));

     */

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
}