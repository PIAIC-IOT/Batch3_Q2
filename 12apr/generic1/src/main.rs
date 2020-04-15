use std::fmt::Display;

fn output(age:u8)-> u8 {
    println!("We got : {}",age);
    age
}

fn print(string_name:String)->String{
    println!("We got : {}",string_name);
    string_name
}

fn generic < T: Display > (data:T)-> T {  //function signature
    println!("we are in generic: {}",data);    
    data
}

fn two_generics < B: Display,V:Display > (data:B,data2:V)-> B {  //function signature
    println!("we are in generic: {}",data);    
    println!("we are in generic: {}",data2);    
    data
}

fn main() {
    let name1 = print("Abdul Rafay".to_string());
    //let name1 = print(22);
    println!("in main: {}",name1);
    let age1 = output(42);
    //let age1 = output("COVID-19".to_string());
    println!("in main: {}",age1);

    let name2 = generic("Abdul Rehman".to_string());
    println!("in main: {}",name2);
    let age2 = generic(23);
    println!("in main: {}",age2);
    let age2 = generic("COVID-19");
    println!("in main: {}",age2);


    let name2 = two_generics("Abdul Rehman".to_string(),22);
    println!("in main: {}",name2);
    let age2 = two_generics(23,"Abdul Rehman".to_string());
    println!("in main: {}",age2);

}
