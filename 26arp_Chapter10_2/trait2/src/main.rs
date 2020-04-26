use rand::Rng;

trait Dice {
    fn roll1(&self) -> usize{
        6
    }
    fn roll2(&self) -> usize {
        1
    }
}

#[derive(Debug)]
struct player1 {
    name : String,
    roll : usize,
}
#[derive(Debug)]
struct player2 {
    name : String,
    roll : usize,
    status : String,
}

impl Dice for player1 {
    fn roll1(&self) -> usize{
        self.roll
    }
    
}

impl Dice for player2 { 
    fn roll2(&self) -> usize{
        self.roll
    }
}

fn main() {
    //let dice = rand::thread_rng().gen_range(1, 7);  
    let p1 = player1 {
        name : "Abdul Rehman".to_string(),
        roll : rand::thread_rng().gen_range(1, 7),
    };
    
    let p2 = player2 {
        name : "Rameez Raja".to_string(),
        roll : rand::thread_rng().gen_range(1, 7),
        status : "Last Winner".to_string()
    };

    println!("Payer 1 {:#?}",p1);
    println!("Own      P1 roll1 {:#?}",p1.roll1());
    println!("Special  P1 roll2 {:#?}",p1.roll2());

    println!("Payer 2 {:#?}",p2);
    println!("Special  P2 roll1 {:#?}",p2.roll1());
    println!("Own      P2 roll2 {:#?}",p2.roll2());

    //println!("Hello, world!");
}
