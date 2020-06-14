fn main() {
    let data  = 2;
    println!("Decimal {}",data);
    println!("Decimal {:b}",data);
    let aftershift = data << 7;
    println!("Decimal {}",aftershift);
    println!("Decimal {:b}",aftershift);
    
    let number  = 10;
    println!("Decimal {}",number);
    println!("Decimal {:b}",number);
    let rightshift = number >> 2;
    println!("Decimal {}",rightshift);
    println!("Decimal {:b}",rightshift);
}

