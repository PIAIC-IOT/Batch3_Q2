fn welcome(){
    println!("Welcome after EID");
}

fn myfunction(data:u8){
    println!("We got data : {}",data);
}

fn main() {
    welcome();
    println!("Hello, world!");
    let age:u8 = 55;
    myfunction(age);
    let myclosure = ||{
        println!("Welcome in my closure");
    };
    myclosure();
    let iot = |mydata|{
        println!("Welcome in IOT closure {}",mydata);
    };
    iot(44);
    let batch3 = ||->u16{
        println!("Welcome in Batch3 closure");
        3000
    };
    println!("we got {} from batch3 closure ",batch3());

    let fee:u16=1500;
    let course = String::from("IOT");
    
    fn quarter3() {
        println!("We are in quarter 3 function");
    }

    let q3=move ||{
        println!("We are in q3 closure {} ",course);
    };
    
    quarter3();
    q3();
    //println!("course is {} ",course);
}
