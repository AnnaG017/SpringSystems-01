const FREEZE_TEMP: f64 = 32.0; //constant 32 for the two functions
fn fahrenheit_to_celsius (f:f64)->f64{
    let conversion_f_to_c = (f-FREEZE_TEMP)*5.0/9.0;
    return conversion_f_to_c;

}
fn celsius_to_fahrenheit (c:f64)->f64{
    let conversion_c_to_f = (c * 9.0/5.0 ) + FREEZE_TEMP;
    return conversion_c_to_f;
}
fn main() {
    let mut temp_f = 32.0; //make it mutable separately 
    let temp_c = fahrenheit_to_celsius (temp_f);
    let temp_f_back = celsius_to_fahrenheit (temp_c);
    println!("{} F = {} C = {} F", temp_f, temp_c, temp_f_back);

  //a for loop for conversion to fahrenheit to celcius back to fahrenheit
   for n in 33..38{
    temp_f = n as f64; //converts integer loop to f64 for calculations
    let f_to_c = fahrenheit_to_celsius (temp_f); //passes temp_f into the function to get Celsius
    let f_back = celsius_to_fahrenheit (f_to_c); //passes f_to_c into the function to get Fahrenheit
     println!("{} F = {} C = {} F", n,f_to_c, f_back);
   }
}