use gamelib::actinator;
//beast
pub struct Beast<'a> {
    name: &'a str,
}
/*
impl Beast {
    fn new (name: &'static str) -> Beast {
        Beast { name: name }
    }
}*/

impl<'a> actinator::Actinator<'a> for Beast<'a> {
    fn new (name: &'a str) -> Beast {
        Beast { name: name }
    }

    fn name (&self) -> &'a str {
        self.name
    }

    fn action (&self) -> &'a str {
        "Beast Does it's own thing!"
    }
}
 #[test]
 fn test_Beast_new() {
     let beast_name         = "I am a beast!";
     let beast: Beast       =  actinator::Actinator::new(beast_name);
     assert!(beast.name == beast_name)
 }

 #[test]
 fn test_Beast_action() {
     let beast_action       = "Beast Does it's own thing!";
     let beast: Beast       = actinator::Actinator::new("The Beast!");
     assert!(beast.action() == beast_action);
 }
