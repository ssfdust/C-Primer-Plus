use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();

    print!("Input your height(cm) and name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    /* let info = input.split(" ")
     *                 .next().and_then(|x| x.trim().parse::<String>()?); */
    let (height, name): (Option<f64>, Option<String>) = {
        let mut iter = input.split(" ");
        (
            iter.next().and_then(|word| word.parse::<f64>().ok()),
            iter.next().and_then(|word| word.parse::<String>().ok()),
        )
    };

    println!("{}, you are {:.3}m tall\n",
           name.unwrap().trim(), height.unwrap());
}
