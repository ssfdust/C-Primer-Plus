#![feature(trace_macros)]

trace_macros!(true);
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
    //
    // 为什么这里是不可以的
    // 是不是因为tmp(), trim(), parse(), unwrap()会被同一个$args解析
    // 那为什么tmp, trim, parse, unwrap 不会被同一个解析
    //
    // 比如 $(args:tt),+ 是可以匹配tmp, trim, parse, unwrap的
    // 问题出在$($args:tt)+是可以匹配tmp(), trim()或者 tmp()所以不清楚
    // 所以有第二版的变异版本
    // 用括号来分开
    //
    // 第二版的变异
    // (test $x:ident, $([$($args:tt)+]).+) => {let $x: String = $($($args)+).+;};
    // 第三版
    (test $x:ident, $($args:tt $bract:tt).*,) => {let $x: String = $($args $bract).*;};
}
//
// macro_rules! test_info {
    // (test $x:ident, $($args:tt $b:tt).*) => {let $x: String = $($args $b).*;};
// }
fn main(){
    // 第二版的变异版本
    // test_info!(test a, [tmp()].[trim()].[parse()].[unwrap()]);
    test_info!(test a, tmp().trim().parse().unwrap(), {});
    
    println!("{}", a);
}

fn tmp() -> String {
    String::from("test")
}
