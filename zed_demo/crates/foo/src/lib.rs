use rand::Rng;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn foo_function() {
    println!("This is a function from foo");
}

pub fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
