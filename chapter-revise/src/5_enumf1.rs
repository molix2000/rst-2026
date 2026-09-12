#[derive(Debug)]
enum Garden {
     Ga11,
     Ga12,
     Ga13,
     Ga14,

}


#[derive(Debug)]
enum Veg {

     Potato(Garden),
     Carrot(Garden),
     Parsnip(Garden),
     Tomato(Garden),

}


fn garden_to_veg(veg: Veg) -> u8 {
   match veg {
       Veg::Potato(garden) => {
         println!("Veg in Garden {garden:#?}");
         12
        }
       Veg::Carrot(garden) => {
         println!("Veg in Garden {garden:#?}");
         10
       }
       Veg::Parsnip(garden) => {
         println!("Veg in Garden {garden:#?}");
         9
       }
       Veg::Tomato(garden) => {
         println!("Veg in Garden {garden:#?}");
         8
       }
         

   }


}


fn main() {

   println!("The Veg to Garden {:#?}", garden_to_veg(Veg::Potato(Garden::Ga11)));
   println!("The Veg to Garden {:#?}", garden_to_veg(Veg::Carrot(Garden::Ga12)));
   println!("the Veg to Garden {:#?}", garden_to_veg(Veg::Parsnip(Garden::Ga13)));
   println!("The Veg to Garden {:#?}", garden_to_veg(Veg::Tomato(Garden::Ga14)));
}
