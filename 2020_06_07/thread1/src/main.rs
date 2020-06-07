use std::thread;
use std::time::Duration;

fn main() {
   
    let add :u32 = 50_000;

    let punjab = thread::spawn(move || {
            for punjab in 0..10 {
                println!("hi number  {} from the spawned thread! we got {}",punjab,add);
                thread::sleep(Duration::from_millis(1000));
            }
     });
     
     for federal in 0..5 {
        println!("hai 2 number {} from the main thread!",federal );
        thread::sleep(Duration::from_millis(1000));
     }

     punjab.join().unwrap();

     
}
