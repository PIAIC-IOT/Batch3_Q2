#[derive(Debug,Clone)]
struct Tech {
    course : String,
//    fee : u32,
}


#[derive(Debug)]
struct Mech <'a> {
    course : &'a str
}

impl<'a> Mech<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.course
    }
    fn understand(&self) {
        println!("understanding: {}", self.course);
    }
}

fn main() {
    let tech1 = Tech { course : "IOT".to_string()};
    println!("{:?}",tech1);
    
    let mech1 = Mech { course : "Mechanical"};
    println!("{:?}",mech1);
    
    mech1.understand();

    println!("we got from method : {}",
    mech1.announce_and_return_part("Last Sunday of Ramazan"));
    let mut memoryreading = "";
    println! ("memoryreading {:?}",memoryreading);
    { // start inner scope
        let name = "Farjad Ali"; 
        println! ("pointer {:p}",&name);
        memoryreading = name;
        println! ("pointer {:p}",&memoryreading);
        println! ("pointer {:p}",memoryreading);
        //let name :&'static str = "Farjad Ali";    //both are same
        println!{"from inner scope {}",name};
    } // end inner scope
    println! ("pointer {:p}",memoryreading);
    println!{"from outer scope {:?}",memoryreading};

    
    let age:u8 = 88;            //for understanding

    

}//end of program