use std::io::{self, Write};

pub fn main() {
    let (mut first_name, mut last_name) = (
        String::new(), String::new()
        );

    let outputs = vec![
        "Input your first name: ".to_owned(),
        "Input your last name: ".to_owned()
        ];

    {
        let mut inputs = vec![&mut first_name, &mut last_name];

        for (i, o) in inputs.iter_mut().zip(outputs.iter()) {
            print!("{}", o);
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut *i).unwrap();
            **i = i.trim().to_owned();
        }

        // enumerate method
        /*  for (idx, o) in outputs.iter().enumerate(){
         *      print!("{}", o);
         *      io::stdout().flush().unwrap();
         *
         *      io::stdin().read_line(&mut *inputs[idx]).unwrap();
         *
         *      *inputs[idx] = inputs[idx].trim().to_owned();
         *  } */
    }

    println!("{}, {}", first_name, last_name);
    println!("{0:1$}, {2:3$}",
             first_name.len(), first_name.len(),
             last_name.len(), last_name.len());
    println!("{}, {}", first_name, last_name);
    println!("{0:<1$}, {2:<3$}",
             first_name.len(), first_name.len(),
             last_name.len(), last_name.len());
}
