fn main(){

    let text = "mazda mx5";
    if text.contains("mx5") {
       println!("Found the correct sports car");

    } 
    if let Some(index) = text.find("mazda") {
        println!("Mazda starts with index {}",index);

    }

    if text.starts_with("mazda") {
       println!("This starts with mazda");
    }

    if text.ends_with("mx5") {

        println!("the text ends with mx5");
    }

    let car_list = "Mazda,Toyota,Ford,Audi";
    let car_selection: Vec<&str> = car_list.split(',').collect();
    println!("{:?}" ,car_selection);

    let bike_list = "Yamaha Suzuki Honda Triumph Ducati Ducati";
    let bike_brands: Vec<&str> = bike_list.split_whitespace().collect();
    println!("{:?}", bike_brands);

    let new_bike_brands = bike_list.replace("Yamaha","RoyalEnfield");
    println!("New Moto brands {}",new_bike_brands);

    let new_brands_list = bike_list.replacen("Ducati", "KTM", 1);
    println!("New brands list {}: ", new_brands_list);


    let shop_brand = "Aldi TKMAX Liddle   #";
    let trimmed_shop_brand = shop_brand.trim();
    println!("{}", trimmed_shop_brand);
    let specific_trimmed_shop_brands = shop_brand.trim_matches('#');
    println!("The specifically trimmed shop brands list is :{}",specific_trimmed_shop_brands); 
}
