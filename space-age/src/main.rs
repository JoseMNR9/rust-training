use std::io;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 0.2408467)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 0.61519726)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / 31557600.0
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 1.8808158)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 11.862615)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 29.447498)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 84.016846)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (31557600.0 * 164.79132)
    }
}

fn main() {
    println!("Enter the number of seconds:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let seconds: u64 = input.trim().parse().expect("Invalid input");

    println!("Enter the planet (Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, or Neptune):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let planet = input.trim();

    let duration = Duration::from(seconds);
    let age = match planet {
        "Mercury" => Mercury::years_during(&duration),
        "Venus" => Venus::years_during(&duration),
        "Earth" => Earth::years_during(&duration),
        "Mars" => Mars::years_during(&duration),
        "Jupiter" => Jupiter::years_during(&duration),
        "Saturn" => Saturn::years_during(&duration),
        "Uranus" => Uranus::years_during(&duration),
        "Neptune" => Neptune::years_during(&duration),
        _ => {
            println!("Invalid planet");
            return;
        }
    };

    println!("Age on {} is {:.2} Earth years", planet, age);
}
