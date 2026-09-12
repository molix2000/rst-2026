#[derive(Debug)]
enum JapaneseCarMakers {
    Toyota(JapaneseCars),
    Mitsubishi(JapaneseCars),
    Nissan(JapaneseCars),
    Mazda(JapaneseCars),
    Lexus(JapaneseCars),
}

#[derive(Debug)]
enum JapaneseCars {
    Supra,
    GTVR4_3000,
    Z_300,
    MX5,
    LBX,
}

impl JapaneseCarMakers {
    fn carto_maker(&self) -> u8 {
        match self {
            JapaneseCarMakers::Toyota(car) => {
                println!("The car most famous from Toyota is {:#?}", car);
                1
            }
            JapaneseCarMakers::Mitsubishi(car) => {
                println!("The car most famous from Mitsubishi is {:#?}", car);
                2
            }
            JapaneseCarMakers::Nissan(car) => {
                println!("The car most famous from Nissan is {:#?}", car);
                3
            }
            JapaneseCarMakers::Mazda(car) => {
                println!("The car most famous from Mazda is {:#?}", car);
                4
            }
            JapaneseCarMakers::Lexus(car) => {
                println!("The car most famous from Lexus is {:#?}", car);
                5
            }
        }
    }
}

fn main() {
    let jcmd = JapaneseCarMakers::Toyota(JapaneseCars::Supra);
    println!("The car from the maker is {:#?}", jcmd);
    let code = jcmd.carto_maker();
    println!("Maker code: {}", code);
}


       
