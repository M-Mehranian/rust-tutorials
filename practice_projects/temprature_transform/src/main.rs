fn main() {
    let degree = 100.0;
    let fresult = fahrenheit_to_celsius(degree);
    println!("{fresult}");

    let cresult = celsius_to_farenheit(degree);
    println!("{cresult}");
}

fn fahrenheit_to_celsius(temprature: f64) -> f64 {
    (temprature - 32.0) / 1.8
}

fn celsius_to_farenheit(temprature: f64) -> f64 {
    (temprature * 1.8) + 32.0
}