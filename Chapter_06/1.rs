const SIZE: usize = 26;

pub fn main() {
    let mut ch: [char; SIZE] = ['0'; SIZE];

    for (idx, c) in ch.iter_mut().enumerate(){
        *c = ('a' as u8 + idx as u8) as char;

        print!("{} ", c);
    }
    println!()
}
