extern crate rand;

//low inclusive - high exclusive returns a string
pub fn random_num (low: i32, high:i32) -> String {
    use self::rand::Rng;//woo! local use of crates...
    self::rand::thread_rng().gen_range(low, high).to_string()
}

#[test]
pub fn test_random_num () {
    assert!(random_num(1000,10000).len() == 4);
}
