pub fn main() {
    let a: f32 = 1.0 / 3.0;
    let b: f64 = 1.0 / 3.0;

    println!("{:.6} {:.6}", a, b);
    println!("{:.12} {:.12}", a, b);
    println!("{:.16} {:.16}", a, b);
}
