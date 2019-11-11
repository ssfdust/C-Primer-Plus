macro_rules! test_info {
    // 第一版 
    //
    // $($args:tt).* = > $($args).*
    //
    // (test $x:ident, $($args:tt).*) => {let $x: String = $($($args)+).*;};
    //
    // error: no rules expected the token `(`
    //   --> test.rs:12:27
    //    |
    // 1  | macro_rules! test_info {
    //    | ---------------------- when calling this macro
    // ...
    // 12 |     test_info!(test a, tmp().trim().parse().unwrap());
    //    |                           ^ no rules expected this token in macro call
    //
    //
    // 第二版 $($($args:tt)+).* =>  $($($args).+).*
    //
    // (test $x:ident, $($($args:tt)+).*) => {let $x: String = $($($args)+).*;};
    //
    // error: local ambiguity: multiple parsing options: built-in NTs tt ('args') or 1 other option.
    //   --> test.rs:25:29
    //    |
    // 25 |     test_info!(test a, tmp().trim().parse().unwrap());
    //    |                             ^
    // error: aborting due to previous error
}

fn main(){
    let a = String::new();

    test_info!(test a, tmp().trim().parse().unwrap());
    
    println!("{}", a);
}

fn tmp() -> String {
    String::from("test")
}
