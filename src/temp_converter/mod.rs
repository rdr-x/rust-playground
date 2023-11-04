use std::io;

fn celcius_to_farnheit(value: f64) -> f64 {
    return (value * (9.00/5.00)) + 32.00;
}

fn farenheit_to_celcius(value: f64) -> f64 {
    return (value - 32.00) * (5.00/9.00);
}

pub fn convert_temp() -> f64 {
    println!("What do you wanna convert? Farenheit or Celcius? ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Expecting either Farenheit or Celcius");

    println!("Introduce the value you want to convert: ");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Expecting a number");

    let value: f64 = value.trim().parse().unwrap();
    match input.to_lowercase().as_str() {
        "farenheit\n" => {
            let result = farenheit_to_celcius(value);
            println!("{}° Farenheit is {}° Celcius", value, result);
            return result;
        },
        "celcius\n" => {
            let result = celcius_to_farnheit(value);
            println!("{}° Celcius is {}° Farenheit", value, result);
            return result;
        },
        _ => {
            println!("Invalid input");
            return 0.0;
        }
    }
}