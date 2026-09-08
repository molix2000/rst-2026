#[derive(Debug)]
enum JapCarModels {
     //Supra,
     //MX5,
     //GTO2000,
     //ZX300,
     MX5,
     GTO3000,
     Supra,
     ZX300,

}




#[derive(Debug)]
enum JapCarManufacturer {
     Toyota(JapCarModels),
     Mazda(JapCarModels),
     Mitsubishi(JapCarModels),
     Nissan(JapCarModels),
}



fn car_models_manufacturers(jcm: JapCarManufacturer) -> u8 {
   match jcm {
       JapCarManufacturer::Toyota(model) => {
           println!("This is the car Toyota produces: {model:?}");
           12
       }
       JapCarManufacturer::Mazda(model) => {
           println!("This is the car Mazda produces: {model:?}");
           13
       }
       JapCarManufacturer::Mitsubishi(model) => {
           println!("This is the car Mitsubishi produces: {model:?}");
           15
       }
       JapCarManufacturer::Nissan(model) => {
           println!("This is the car Nissan produces: {model:?}");
           18
       }
   }
}

fn main() {
      println!("This is the Toyota part {:#?}", car_models_manufacturers(JapCarManufacturer::Toyota(JapCarModels::Supra)));
      println!("This is the Mazda part {:#?}", car_models_manufacturers(JapCarManufacturer::Mazda(JapCarModels::MX5)));
      println!("This is the Mitsubishi part {:#?}", car_models_manufacturers(JapCarManufacturer::Mitsubishi(JapCarModels::GTO3000)));
      println!("This is the Nissan part {:#?}", car_models_manufacturers(JapCarManufacturer::Nissan(JapCarModels::ZX300)));
}


