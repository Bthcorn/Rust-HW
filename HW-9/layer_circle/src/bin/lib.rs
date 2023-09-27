use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64, 
}

#[derive(Debug, Clone)]
pub struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>
}
fn gen_circle<R: Rng>(rng: &mut R, name: String, color: String ) -> Layer {
    let mut result = Vec::new();

    let n = rng.gen_range(20..=50);

    for _ in 0..n {
        let x = rng.gen_range(-100.0..=100.0); 
        let y = rng.gen_range(-100.0..=100.0);
        let r = rng.gen_range(-10.0..20.0);
        result.push(Circle { x, y, r });
    }  

    Layer {
        name: name,
        color: color,
        objects: result,
    }
}

fn random_rgb_color() -> String {
    let mut rng = rand::thread_rng();

    // Generate random values for red, green, and blue components
    let a = rng.gen_range(0..256);
    let b = rng.gen_range(0..256);
    let c = rng.gen_range(0..256);
    let d = rng.gen_range(0..256);

    // Format the RGB color as a hexadecimal string
    format!("#{:02X}{:02X}{:02X}{:02X}", a, b, c, d)
}

fn gen_obj_layer_list<R: Rng>(rng : &mut R, n: i32) -> Vec<Layer> {
    let mut result = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 1..n + 1 {
        let name = format!("Layer {i}");
        let color = random_rgb_color();
        let generated_obj = gen_circle(&mut rng, name, color);
        result.push(generated_obj);
    }
    result
}

#[test]
fn test_gen_obj_layer_list() {
    let mut rng = rand::thread_rng();

    let expected_layer_count = 5;
    let generated_layer = gen_obj_layer_list(&mut rng, expected_layer_count);

    for layer in generated_layer {
        assert!(!layer.name.is_empty());
        assert_eq!(layer.color.len(), 9);
        assert!(layer.objects.len() >= 20 && layer.objects.len() <= 50);
    }
}

pub const PI: f64 = 3.14;
fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
    let mut result = Vec::new();

    for layer in layers {
        let mut total_area = 0.0;
        let mut count = 0;
        // let mut min_area = 0.0 or = total_area
        // let mut max_area = 0.0 or = total_area
        for circle in &layer.objects {
            let area = PI*(circle.r).powf(2.0);
            total_area += area;
            count += 1;
        }
        let average_area = total_area / count as f64;
        result.push((layer.name.clone(), average_area))
    }
    result
}

#[test]
fn test_cal_average_area() {
    let mut rng = rand::thread_rng();
    let generated = gen_obj_layer_list(&mut rng, 4);
    let calculation = cal_average_area(&generated);
    for area in calculation {
        assert!(area.1 >= 0.0 && area.1 <= PI*(20.*20.));
        assert!(!area.0.is_empty());
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let x = gen_obj_layer_list(&mut rng, 4);
    let calculation = cal_average_area(&x);
    println!("{:?}", x );
    println!("{:?}", calculation);
}