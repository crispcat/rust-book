mod temperature;

use temperature::Degree;
use temperature::Temperature;

const UOM_INPUT_PHRASE: &str = "\nType units of measure.\n\
    C - for °Celsius\n\
    F - for °Fahrenheit\n\
    K - for °Kelvin\n";

fn main() {

    println!("\nHi! I can convert temperature.");

    loop {

        println!("\nEnter temperature as a float number:\n");

        let val: f64 = loop {
            match utils::stdin_read_line().trim().parse() {
                Ok(f) => break f,
                Err(_) => { println!("\nEnter a float number:\n"); }
            }
        };

        println!("{}", UOM_INPUT_PHRASE);
        let temperature: Temperature = loop {
            match utils::stdin_read_line().trim() {
                "C" => { break Temperature::create(val, Degree::Celsius) },
                "F" => { break Temperature::create(val, Degree::Fahrenheit) },
                "K" => { break Temperature::create(val, Degree::Kelvin) },
                _ => { println!("{}", UOM_INPUT_PHRASE); }
            }
        };

        print_temperature_table(temperature);
    }
}

fn print_temperature_table(temperature: Temperature) {
    println!("\n{}", temperature.to_string());

    if !temperature.is(Degree::Celsius) {
        println!("{}", temperature.convert_to(Degree::Celsius).to_string())
    }

    if !temperature.is(Degree::Fahrenheit) {
        println!("{}", temperature.convert_to(Degree::Fahrenheit).to_string())
    }

    if !temperature.is(Degree::Kelvin) {
        println!("{}", temperature.convert_to(Degree::Kelvin).to_string())
    }
}