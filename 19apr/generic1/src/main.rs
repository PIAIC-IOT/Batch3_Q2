/////our code
fn main () {
    let integer = Some(5);
    let float = Some(5.0);
}    

enum Option <T> {
    some(T),
    None,
}

/////Compiler COde
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

// use std::ops::Add;
// use std::fmt::Display;
// //Generic function with Two parameters
// // fn generic_double<T,K>(msg1:T, msg2:K) -> (T,K){
// //     (msg1+ msg2)//Returning as a tuple
// // }

// fn generic_double<T:Add,K:Add>(msg1:T, msg2:K)  {
//     let d = msg1;
//     let e = msg2;
//     let r = d+e;
//     //println!("{}",r);
// }

// fn main () {
//     // let result = generic_double(20, 31);
//     // println!("{:?}",result);
//     generic_double(20, 31);
// }