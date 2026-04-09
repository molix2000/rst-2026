pub fn tempconvert() {
    // Formulas:
    // Convert Fahrenheit to Celsius: ºC = (ºF - 32)(0.5556)
    // Convert Celsius to Fahrenheit: ºF = (ºC * 1.8) + 32
    let  feh: i64 = 22;
    let  cel: i64 = (feh - 32);

    let celtofeh: f64  = 40.0;
    let fehfromcel: f64 = (celtofeh as f64 * 1.8);
    //let mut fehfromcel: f64 = celtofeh * 1.8;
    println!("CEL temp for Feh valued at {feh} is {cel}");
    println!("FEH temp for Cel valued at {celtofeh} is {fehfromcel}");
}
