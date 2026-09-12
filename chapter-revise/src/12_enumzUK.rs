#[derive(Debug)]

enum Counties {
    Kent,
    Berkshire,
    Lincolinshire,
    Derbisher(Cities),
}

#[derive(Debug)]

enum Cities {
     Canterbury,
     Reading,
     Lincolin,
     Derby,
}

impl Counties{

    fn anycounty(&self) -> u8 {
         match self {
           Counties::Kent => 1,
           Counties::Berkshire => 2,
           Counties::Lincolinshire => 3,
           Counties::Derbisher(city) => {
             println!("The city fom the county is {:#?}", city);
             55
           
             }
        }
    }
    
}

fn main () {

         let county01 = Counties::Derbisher(Cities::Derby);  
         println!("The county and city are numbered {:#?} ", county01.anycounty());


}


