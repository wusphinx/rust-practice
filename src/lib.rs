pub fn fibonacci(x :u64) -> u64{
    if x <= 1{
        return x 
    }
    return fibonacci(x-1) + fibonacci(x-2)
}

#[cfg(test)]
mod fibonacci_tests {
    use super::fibonacci;
    #[test]
    fn it_works() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
    }
}