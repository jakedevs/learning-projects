use std::io;

fn main() {
    let mut input_temp = String::new();
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to get input.");

    let input_temp: f64 = input_temp.trim().parse().expect("Input must be a number.");

    println!("Convert to: F/C \nExit: Q");

    let mut input_conversion_unit = String::new();
    io::stdin()
        .read_line(&mut input_conversion_unit)
        .expect("Failed to get input.");

    let fahrenheit = (input_temp * 9.0 / 5.0) + 32.0;
    let celsius = (input_temp - 32.0) * 5.0 / 9.0;

    println!("Conversion unit: {input_conversion_unit}");
    match input_conversion_unit.trim() {
        "F" => {
            println!("{fahrenheit}");
        }
        "C" => {
            println!("{celsius}");
        }
        "Q" => {}
        _ => {
            println!("Choose one of: F/C/Q")
        }
    };
}
