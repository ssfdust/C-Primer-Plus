const ROWS: i32 = 6;

pub fn main() {
    let mut ch = 'A';

    for i in 0..ROWS {
        for _ in 0..(i + 1) {
            print!("{}", ch);
            ch = (1u8 + ch as u8) as char;
        }
        println!("");
    }
}
