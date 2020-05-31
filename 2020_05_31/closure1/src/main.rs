use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn game(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(2));
    println!("Hello, world! after 2 second");
    
    let mut myinstance = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("{:?}",myinstance.value);
    myinstance.game(100);
    println!("{:?}",myinstance.value);
    myinstance.game(200);
    println!("{:?}",myinstance.value);
    let data = (myinstance.calculation)(500);
    println!("{}",data);
}
