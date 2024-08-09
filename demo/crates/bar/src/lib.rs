use foo::foo_function;

pub fn bar_function() {
    println!("This is a function from bar library");
    foo_function(); // 调用 foo 中的函数
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
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
