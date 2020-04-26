trait Printing {
    fn tppt(&self)->String;
}

trait Looking {
    //by defauly
    fn Lppt(&self)->String {
        format!("From Looking Default: Looking at you for some food")
    }
}

#[derive(Debug)]
struct Upper {
    MainCourse : String,
    Drink      : String,
    Oxygen     : bool,
    Water      : u8,
}
#[derive(Debug)]
struct Middle {
    Paratha : String,
    Drink  : String,
    Oxygen : bool,
    Water  : u8,
}
#[derive(Debug)]
struct Lower {
    Oxygen : bool,
    Water  : u8,
}

impl Upper {
    fn ppt(&self) {
        println!("{:#?}",self);
    }
}

impl Middle {
    fn ppt(&self) {
        println!("{:#?}",self);
    }
}

impl Lower {
    fn ppt(&self) {
        println!("{:#?}",self);
    }
}

impl Printing for Upper {
    fn tppt(&self)->String{
        format!("From Printing : {} + {} ",&self.MainCourse,&self.Drink)
    }
}


impl Printing for Lower {
    fn tppt(&self)->String{
        format!("From Printing : {} + {} ",&self.Oxygen,&self.Water)
    }
}


impl Printing for Middle {
    fn tppt(&self)->String{
        format!("From Printing : {} + {} ",&self.Paratha,&self.Drink)
    }
}

impl Looking for Upper {
    fn Lppt(&self)->String{
        format!("From Looking : {} + {} ",&self.MainCourse,&self.Drink)
    }
}

impl Looking for Lower { }


fn main() {
    let dha = Upper {
        MainCourse : "Donut".to_string(),
        Drink      : "Coffee".to_string(),
        Oxygen     : true,
        Water      : 3,
    };

    let mid = Middle {
        Paratha : "Sadha Paratha".to_string(),
        Drink  : "Chai".to_string(),
        Oxygen : true,
        Water  : 5,
    };
    
    let low = Lower {
        Oxygen : true,
        Water  : 4,
    };

    dha.ppt();
    mid.ppt();
    low.ppt();

    println!("{:#?}",dha.tppt());
    println!("{:#?}",dha.Lppt());
    println!("{:#?}",low.Lppt());
}
