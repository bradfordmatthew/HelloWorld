extern crate rand;

pub fn horrid () -> String {
    use self::rand::Rng;
    self::rand::thread_rng().gen_range(1000, 10000).to_string()
}

#[test]
pub fn test_horrid () {
    assert!(horrid().len() == 4);
}
