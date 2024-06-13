use std::io;
use math::round;

#[derive(Copy, Clone)]
enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin
}

#[derive(Copy, Clone)]
struct Temperature {
    val : f64,
    deg : Degree
}

impl Temperature {

    fn convert_to(&self, deg: Degree) -> Temperature {

        match self.deg {
            Degree::Celsius => match deg {
                Degree::Celsius => self.clone(),
                Degree::Fahrenheit => Temperature { deg: Degree::Fahrenheit, val: self.val * 1.8 + 32. },
                Degree::Kelvin => Temperature { deg: Degree::Kelvin, val: self.val + 273.15 }
            },
            Degree::Fahrenheit => match deg {
                Degree::Celsius => Temperature { deg: Degree::Celsius, val: (self.val - 32.) / 1.8 },
                _ => self.convert_to(Degree::Celsius).convert_to(deg)
            },
            Degree::Kelvin => match deg {
                Degree::Celsius => Temperature { deg: Degree::Celsius, val: self.val - 273.15 },
                _ => self.convert_to(Degree::Celsius).convert_to(deg)
            }
        }
    }

    fn to_string(&self) -> String {
        let prefix = match self.deg {
            Degree::Celsius     =>  "Celsius    : ",
            Degree::Fahrenheit  =>  "Fahrenheit : ",
            Degree::Kelvin      =>  "Kelvin     : ",
        };

        let val = round::half_up(self.val, Self::DECIMAL_DIGITS).to_string();

        let postfix = match self.deg {
            Degree::Celsius     =>  "° C",
            Degree::Fahrenheit  =>  "° F",
            Degree::Kelvin      =>  " K",
        };

        format!("{}{}{}", prefix, val, postfix)
    }

    const DECIMAL_DIGITS: i8 = 3;
}

const UOM_INPUT_PHRASE: &str = "\nType units of measure.\n\
    C - for °Celsius\n\
    F - for °Fahrenheit\n\
    K - for °Kelvin\n";

fn main() {

    println!("\nHi! I can convert temperature.");

    loop {

        println!("\nEnter temperature as a float number:\n");

        let temperature: f64 = loop {
            match stdin_read_line().trim().parse() {
                Ok(f) => break f,
                Err(_) => { println!("\nEnter a float number:\n"); }
            }
        };

        println!("{}", UOM_INPUT_PHRASE);
        let temperature: Temperature = loop {
            match stdin_read_line().trim() {
                "C" => { break Temperature { val: temperature, deg: Degree::Celsius } },
                "F" => { break Temperature { val: temperature, deg: Degree::Fahrenheit } },
                "K" => { break Temperature { val: temperature, deg: Degree::Kelvin } },
                _ => { println!("{}", UOM_INPUT_PHRASE); }
            }
        };

        print_temperature_table(temperature);
    }
}

fn print_temperature_table(temperature: Temperature) {

    println!("\n{}", temperature.to_string());

    if !matches!(temperature.deg, Degree::Celsius) {
        println!("{}", temperature.convert_to(Degree::Celsius).to_string())
    }

    if !matches!(temperature.deg, Degree::Fahrenheit) {
        println!("{}", temperature.convert_to(Degree::Fahrenheit).to_string())
    }

    if !matches!(temperature.deg, Degree::Kelvin) {
        println!("{}", temperature.convert_to(Degree::Kelvin).to_string())
    }
}

fn stdin_read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read stdin");
    line
}