use num_traits::Float;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: f64,
    diameter: f64,
    area: f64,
}

impl Circle {
    fn new_with_area(area: f64) -> Self {
        Circle {
            diameter: area / std::f64::consts::PI,
            radius: (area / std::f64::consts::PI).sqrt(),
            area: area,
        }
    }

    fn new_with_radius(radius: f64) -> Self {
        Circle {
            diameter: radius * 2.0,
            radius: radius,
            area: std::f64::consts::PI * radius.powf(2.0),
        }
    }

    fn new_with_diameter(diameter: f64) -> Self {
        Circle {
            diameter: diameter,
            radius: diameter / 2.0,
            area: std::f64::consts::PI * (diameter / 2.0).powf(2.0),
        }
    }
}

fn get_number_from_io<T: Float + FromStr>(prompt: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    println!("{}", prompt);

    let mut text = String::from("");

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let num: T = text
        .trim()
        .parse()
        .expect("Something went wrong durung parsing I/O.");

    return num as T;
}

fn main() {
    println!("1. Area");
    println!("2. Radius");
    println!("3. Diameter");

    let option: f64 = get_number_from_io("Pick an Option:");

    match option {
        1.0 => {
            let area: f64 = get_number_from_io("Enter Area: ");
            println!("{:#?}", Circle::new_with_area(area));
        }

        2.0 => {
            let radius: f64 = get_number_from_io("Enter Radius: ");
            println!("{:#?}", Circle::new_with_radius(radius));
        }

        3.0 => {
            let diameter: f64 = get_number_from_io("Enter Diameter: ");
            println!("{:#?}", Circle::new_with_diameter(diameter));
        }

        _ => {
            println!("Not a valid option :(");
        }
    }
}
