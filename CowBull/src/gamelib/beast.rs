use std::borrow::Cow;
use gamelib::actinator;
//beast
pub struct Beast<'a> {
    name: &'a str,
}

impl<'a> Beast<'a> {
    //fn new (name: &'a str) -> Beast<'a> {
    //    Beast { name: name }
    //}
}

impl<'a> actinator::Actinator<'a> for Beast<'a> {
    fn new (name: &'a str) -> Beast<'a> {
        Beast { name: name }
    }

    fn name (&self) -> &'a str {
        self.name
    }

    fn action(&self)-> Cow<'a, str> {
        let some_string: String = format!("Beast {} is a beastly beast!", self.name()).to_string();
        return Cow::Owned(some_string);
    }
}

 #[test]
 fn test_Beast_new() {
     let beast_name         = "I am a beast!";
     let beast: Beast       =  actinator::Actinator::new(beast_name);
     assert!(beast.name     == beast_name)
 }
