use std::fmt::Display;
//use std::ops::Output;
use std::ops::Add;

#[derive(Debug)]
struct data <T,V> {
    one : T,
    Two : V,
}

fn main() {
    println!("Hello, world! {}",add(23,-19));
    let abc = data {
        one: 55,
        Two: 66.6,
    };
    println!("{}",abc.one+abc.Two as i32);
    //println!("{}",second(Some(3),Some(4.4)));
    // second(Some(3),Some(4.4));
    let two = Point { x:5,y:5};
    let three = Point { x:6,y:6};
    println!("{:?}",two);
    println!("{:?}",two.add(three));
}

// fn second<T:Option(T)> (one:T,two:T){
//     println!("{:?}",one);
// }
// fn second<T,U> (one:T,two:U) -> i32 {
//     let a = one as i32;
//     let b = two as i32;
//     let sum = a+ b;
//     sum
// }

fn add<T:Display+Add> (var1:T,var2:T)-> T::Output {
    var1+var2
}

#[derive(Debug, PartialEq)]
struct Point<T,U> {
    x: T,
    y: U,
}

impl Add for Point<T,U> {
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}