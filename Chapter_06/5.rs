const ROWS: i32 = 5;

pub fn main() {
    let ch = 'A';

    for i in 1..(ROWS + 1) {
        for _ in 0..(ROWS - i) {
            print!(" ");
        }
        for j in 0..i {
            print!("{}", (ch as u8 + j as u8) as char);
        }
        for j in (0..(i - 1)).rev() {
            print!("{}", (ch as u8 + j as u8) as char)
        }
        println!();
    }
}
