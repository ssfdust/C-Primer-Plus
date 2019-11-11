pub fn main() {
    for i in 1..7 {
        for j in 0..i{
            print!("{}", ('F' as u8 - j as u8) as char);
        }
        println!()
    }
}
