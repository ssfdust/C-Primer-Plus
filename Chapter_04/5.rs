use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();

    print!("Input the download speed(Mb/s) and the file size(MB): ");
    io::stdout().flush().ok();

    io::stdin().read_line(&mut input).ok();

    let input_vec: Vec<f64> = input.split(" ")
                    .map(|x| x.trim().parse().unwrap()).collect::<Vec<f64>>();

    let (speed, size) = (input_vec[0], input_vec[1]);
    let time = size / speed * 0.8;

    println!("At {:.2} megabits per second, a file of {:.2} megabytes",
             speed, size);
    println!("downloads in {:.2} seconds.\n",time);
}
