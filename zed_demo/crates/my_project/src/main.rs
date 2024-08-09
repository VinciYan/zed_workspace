use bar::bar_function;
use foo::generate_random_number;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", bar::add_one(num));
    bar_function();
    let random_number = generate_random_number();
    println!("Random number generated in bar: {}", random_number);
}