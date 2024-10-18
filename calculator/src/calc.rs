use num_traits::ToPrimitive;

pub struct Calc;

impl Calc {
    pub fn add<T: ToPrimitive, U: ToPrimitive>(num1: T, num2: U) -> f64 {
        let num1 = num1.to_f64().expect("Parsing error!");
        let num2 = num2.to_f64().expect("Parsing error!");

        num1 + num2
    }

    pub fn sub<T: ToPrimitive, U: ToPrimitive>(num1: T, num2: U) -> f64 {
        let num1 = num1.to_f64().expect("Parsing error!");
        let num2 = num2.to_f64().expect("Parsing error!");

        num1 - num2
    }

    pub fn mul<T: ToPrimitive, U: ToPrimitive>(num1: T, num2: U) -> f64 {
        let num1 = num1.to_f64().expect("Parsing error!");
        let num2 = num2.to_f64().expect("Parsing error!");

        num1 * num2
    }

    pub fn div<T: ToPrimitive, U: ToPrimitive>(num1: T, num2: U) -> f64 {
        let num1 = num1.to_f64().expect("Parsing error!");
        let num2 = num2.to_f64().expect("Parsing error!");

        num1 / num2
    }

    pub fn modulus<T: ToPrimitive, U: ToPrimitive>(num1: T, num2: U) -> f64 {
        let num1 = num1.to_f64().expect("Parsing error!");
        let num2 = num2.to_f64().expect("Parsing error!");

        num1 % num2
    }
}
