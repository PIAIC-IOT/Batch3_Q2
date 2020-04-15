#[derive(Debug)]
struct cube <T,U,V> {
    width : T,
    height : V,
    length : U,
}


fn main() {
    let cube1 = cube {
        width : 5,
        height : 5.5,
        length : "5 feet".to_string(),
    };
    println!("{:#?}",cube1);    
    let cube2 = cube {
        width : "5 feet".to_string(),
        height : "5 feet".to_string(),
        length : 22,
    };
    println!("{:#?}",cube2);    
}
