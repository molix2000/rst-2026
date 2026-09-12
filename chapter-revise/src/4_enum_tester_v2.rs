use std::collections::btree_map;

enum BikeTypes {
     Electric,
     Hybrid,
     ThreeWheels,
}



enum BikeNames {
     Brompton(BikeTypes),
     Tmz(BikeTypes),
     UthaBikes(BikeTypes),
}


fn bike_name_type(bn: BikeNames)->u8 {

    match bn {

        BikeNames::Brompton(BikeTypes::Electric) => {
           println!("The bike, {bn:#?}");
           23
        }    
        
        BikeNames::Tmz(BikeTypes::Hybrid) => {
           println!("This bike, {bn:#?}");
           11
        }
        
        BikeNames::Utha_Bikes(BikeTypes::ThreeWheels) => {
           println!("This bike, {bn:#?}");
           21
        }



   }

}


fn main() {
   
   println!("This is the Bike {:#?}", bike_name_type(BikeNames::Brompton(BikeTypes::Electric)));
   println!("This is the Bike {:#?}", bike_name_type(BikeNames::Tmz(BikeTypes::Hybrid)));
   println!("This is the Bike {:#?}", bike_name_type(BikeNames::UthaBikes(BikeTypes::ThreeWheels)));

}
