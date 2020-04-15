fn main() {
    let age = [22,33,44,55];
    //index    0  1  2  3
    println!("we got : {:?}",age.get(2));
    let disease = ["Flu","Cough","Pneumia","Fever","COVID-19"];
    //               0      1      2         3        4
    println!("we got : {:?}",disease.get(5));
    println!("Length of Array is : {:?}",disease.len());
    let name = "Faisal".to_string();
    println!("Length of {} is {}",name,name.len());
}
