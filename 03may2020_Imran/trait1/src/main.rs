trait Bill {
    fn yourbill(&self)->String;
}
trait free {
    fn nopayment(&self)->String{
        format!("Your bill waived, due to COVID-19, ")
    }
}
#[derive(Debug)]
struct KFC {
    food: String,
    price : u32,
    quantity: u8,
}
#[derive(Debug)]
struct McDonald {
    foodname: String,
    price : u32,
    drink : String,
}
#[derive(Debug)]
struct PizzaHut {
    pizza: String,
    price : u32,
    size : u32,
}
#[derive(Debug)]
struct PC {
    tea:String,
    price:u8,
}
impl Bill for McDonald {
    fn yourbill(&self)->String{
        format!("McDonald Bill, Food : {} drink : {} Total Bill : {} ",self.foodname,self.drink,self.price)
    }
}
impl Bill for PizzaHut {
    fn yourbill(&self)->String{
        format!("PizzHut Bill, Pizza : {} Size : {} Total Bill : {} ",self.pizza,self.size,self.price)
    }
}


impl free for PizzaHut {}
impl free for PC {}
impl free for McDonald {}

impl free for KFC {
    fn nopayment(&self)->String{
        format!("KFC Bill, Food {}, quantity : {} , Total Bill : {}",self.food,self.quantity,self.price)
    }
}

fn printingfunction(age:u8){

}
fn printintdata(data1:McDonald){

}
fn place_order(data: impl free ) {
    println!("We are in place order");
    println!("{}",data.nopayment());
}

// fn place_order <U:free> (data: U ) {
//     println!("We are in place order");
//     println!("{}",data.nopayment());
// }

// fn take_away <T: Bill> (data: T ) {
//     println!("We are in take away");
//     println!("{}",data.yourbill());
// }

fn take_away <T> (data: T ) 
where T: Bill {
    println!("We are in take away");
    println!("{}",data.yourbill());
}

fn ret_trait()-> impl Bill {
    println!("we are in return Trait");
    PizzaHut {
        pizza: "Arabian Ranch".to_string(),
        price : 2000,
        size : 9,
    }
}

fn main() {
    let kfcfood = KFC {
        food: "Mighty Zinger Burger".to_string(),
        price : 1000,
        quantity: 1,
    };
    let mcfood =  McDonald {
        foodname: "Big Mag".to_string(),
        price : 1200,
        drink : "CocaCola".to_string(),
    };
    let pizafood=  PizzaHut {
        pizza: "Hawain Pizza".to_string(),
        price : 1500,
        size : 12,
    };
    let tea =  PC {
        tea:"Milk Tea".to_string(),
        price:40,
    };
    
    println!("yourbill :  {}",mcfood.yourbill());
    println!("yourbill :  {}",pizafood.yourbill());
    println!("yourbill :  {}",pizafood.nopayment());
    println!("nopayment : {}",tea.nopayment());
    println!("nopayment : {}",kfcfood.nopayment());
    println!("");
    place_order(kfcfood);
    place_order(tea);
    take_away(pizafood);
    let data = ret_trait();
    
    println!("{:#?}",data.yourbill());
    
}
