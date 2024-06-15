use std::fmt::{Display, Formatter};
use math::round;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin
}

#[derive(Copy, Clone)]
pub struct Temperature {
    val : f64,
    deg : Degree
}

impl Temperature {

    pub fn create(val: f64, deg: Degree) -> Temperature {
        Temperature { val, deg }
    }

    pub fn is(&self, deg: Degree) -> bool {
        self.deg == deg
    }

    pub fn convert_to(&self, deg: Degree) -> Temperature {

        match self.deg {
            Degree::Celsius => match deg {
                Degree::Celsius => self.clone(),
                Degree::Fahrenheit => Temperature { deg, val: self.val * 1.8 + 32. },
                Degree::Kelvin => Temperature { deg, val: self.val + 273.15 }
            },
            Degree::Fahrenheit => match deg {
                Degree::Celsius => Temperature { deg, val: (self.val - 32.) / 1.8 },
                _ => self.convert_to(Degree::Celsius).convert_to(deg)
            },
            Degree::Kelvin => match deg {
                Degree::Celsius => Temperature { deg, val: self.val - 273.15 },
                _ => self.convert_to(Degree::Celsius).convert_to(deg)
            }
        }
    }
}

const DECIMAL_DIGITS: i8 = 3;

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let prefix = match self.deg {
            Degree::Celsius     =>  "Celsius    : ",
            Degree::Fahrenheit  =>  "Fahrenheit : ",
            Degree::Kelvin      =>  "Kelvin     : ",
        };

        let val = round::half_up(self.val, DECIMAL_DIGITS).to_string();

        let postfix = match self.deg {
            Degree::Celsius     =>  "° C",
            Degree::Fahrenheit  =>  "° F",
            Degree::Kelvin      =>  " K",
        };

        write!(f, "{}{}{}", prefix, val, postfix)
    }
}