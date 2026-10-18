// Convert Celsius to Fahrenheit and Kelvin.
fn main() {
    let celsius = 24.0_f64;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let kelvin = celsius + 273.15;

    println!("{celsius:.1}°C = {fahrenheit:.1}°F");
    println!("{celsius:.1}°C = {kelvin:.2} K");
}
