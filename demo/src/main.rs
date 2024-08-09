use bar::bar_function;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", bar::add_one(num));
    bar_function();
}