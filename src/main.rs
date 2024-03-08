//mod temp_converter;
mod fibonacci;

fn main() {
    //temp_converter::convert_temp();
    let n = 9;
    println!("Fibonnaci value at {}th position is: {}",n, fibonacci::generate_at_position(n));
}
