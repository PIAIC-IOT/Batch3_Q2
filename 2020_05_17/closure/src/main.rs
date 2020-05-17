fn sunday(){
    
    println!("Welcome Sunday ");
}
//same closure
// ||{
//     println!("Welcome Monday");
// }

fn rect(height:u8){
    println!("Height is : {}",height);
}
//same closure
// |height|{
//     println!("Welcome Monday");
// }

fn sum()->i32{
    88
}

//same closure
// ||sum{
//      println!("Welcome Monday");
//      88
// }

fn main() {
    sunday();
    rect(77);
    println!("{}",sum());

    let monday = ||{
        println!("We are in closure");
        println!("Welcome Monday");
    };

    monday();

    let tuesday = || println!("We are in Tuesday closure");
    tuesday();

    let rect2 = |height|{
        println!("We are in rect 2 closure {} ",height);
        // for data in 0..=height {
        //     println!("{}",data);
        // }
    };

    rect2(2);
    rect2(20);

    
    let sum2 = ||->u8{
        println!("We are in sum2 closure");
        50
    };

    println!("{}",sum2());

    let sum3 = |data|data;

    println!("{}",sum3(3));

    let sum4 = |one:u8,two:u8|->u8{
        println!("We are in sum4 closure");
        one+two
    };
    println!("{}",sum4(5,25));

   let age:u8 = 22;
   println!("Age {}",age);

   let environment = || { 
       println!("We can access environment variable {}",age);
    };
    environment();
    println!("Age {}",age);

    let salary = "140_000".to_string();
   println!("Salary {}",salary);

   let move1 = move || { 
       println!("We can access environment variable {}",salary);
    };
    move1();
    //println!("Salary {}",salary);

}