fn main() {
    let first : u8 = 44 ;
    println!("Decimal value of first is {}",first);
    println!("binary value of first is {:b}",first);
    let second : u8 = 88 ;
    println!("decimal value of second is {}",second);
    println!("binary value of second is {:b}",second);
    let result = first ^ second;
    println!("decimal value of result is {}",result);
    println!("binary value of result is {:b}",result);

    let third : u8 = 129 ;
    println!("decimal value of third is {}",third);
    println!("binary value of third is {:b}",third);
    let mut result1 :u8;
     result1 = !third;
    println!("decimal value of result1 is {}",result1);
    println!("binary value of result1 is {:b}",result1);

}
