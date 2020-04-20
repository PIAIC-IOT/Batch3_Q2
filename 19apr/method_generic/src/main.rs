use std::fmt::Display;
#[derive(Debug)]
struct Vehicle <N,R> {
    model : String,
    make : String,
    engine : N,
    year : R
}

// struct Vehicle {
//     model : String,
//     make : String,
//     engine : u32,
//     year : u32,
// }

impl <N:Display,R:Display> Vehicle <N,R> {
    fn ppt(&self) -> &N {
        println!("We are in method",);
        println!("model : {}",self.make);
        println!("model : {}",self.engine);
        println!("model : {}",self.year);
        // let name = "Noman".to_string();
        // &name
        &self.engine
    }
    fn mix<X,Y>(self,other:Vehicle<X,Y>) ->Vehicle<N,Y> {
//     engine,year                     
      //u32,i32                   u32,i32
//         N R  (self)
//     engine,year                     
//       f32,u8
        Vehicle {
            model  : "Ciaz + Etron".to_string(),            
            make   : "Suzuki + Audi".to_string(),
            engine : self.engine,
            year   : other.year

        }

    }
}
fn main() {
    let e1 :f32 = 1600.00;
    let y1 : u8 = 20;

    let v1 = Vehicle {
        model : "Ciaz".to_string(),
        make : "Suziki".to_string(),
        engine : e1,
        year : y1,
    };

    let e2 :u32 = 2000;
    let y1 = "COVID".to_string();

    let v2 = Vehicle {
        model : "E Tron".to_string(),
        make : "Audi".to_string(),
        engine : e2,
        year : y1
    };
    // println!("{:#?}",v1);
    // println!("{:#?}",v2);
    println!("{:#?}",v1);
    println!("{:?}",v1.ppt());
    println!("{:?}",v2.ppt());
    
    let mix_vehicle = v1.mix(v2);
    println!("{:#?}",mix_vehicle);
}

// //Generic function with Two parameters
// fn generic_double<T,K>(msg1:T, msg2:K) -> (T,K){
//     (msg1, msg2)//Returning as a tuple
// } 