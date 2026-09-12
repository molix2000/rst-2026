#[derive(Debug)]
enum JapBikes {
     Honda,
     Yamaha,
     Suzuki,
     Kawazaki,
    
}

#[derive(Debug)]
enum JapBikeModels {
     CB500F(JapBikes),
     YZFR1S(JapBikes),
     SV650S(JapBikes),
     Ninja600(JapBikes),
}



   fn bike_type_cost(jbm: JapBikeModels) -> u8 {

       match jbm {
           JapBikeModels::CB500F(jb) => {
             println!("This bike is {jb:?}");
             45
            }
           JapBikeModels::YZFR1S(jb) => {
               println!("This bike is {jb:?}");
               33
            }
           JapBikeModels::SV650S(jb) => {
               println!("This bike is {jb:?}");
               31
            }
           JapBikeModels::Ninja600(jb) => {
               println!("This bike is {jb:?}");
               20
            }

       }
       
    }


fn main() { 
    println!("This is the implementation of bike_type_cost");
    println!("This is the Honda component {:?}", bike_type_cost(JapBikeModels::CB500F(JapBikes::Honda)));
    println!("This is the Yamaha bike comp {:?}",bike_type_cost(JapBikeModels::YZFR1S(JapBikes::Yamaha)));
    println!("This is the Suzuki bike comp {:?}",bike_type_cost(JapBikeModels::SV650S(JapBikes::Suzuki)));
    println!("This is the Kawasaki bike comp {:?}",bike_type_cost(JapBikeModels::Ninja600(JapBikes::Kawazaki)));
}
