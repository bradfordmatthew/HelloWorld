use gamelib::actinator;
use gamelib::object;

pub struct Humanoid<'a> {
    pub name: &'static str,
    pub pack: Vec<&'a object::GameObject<'a>>,
}

impl<'a> Humanoid<'a> {
    pub fn new (name: &'static str ) -> Humanoid {
        Humanoid {
            name: name ,
            pack: Vec::new(),
        }
    }

    pub fn add_to_pack (&mut self, w: &'a object::GameObject) {
        self.pack.push(w);
    }

    pub fn remove_from_pack (&mut self) -> &object::GameObject {
        self.pack.pop().unwrap()
    }
}

impl<'a> actinator::Actinator for Humanoid<'a> {
    fn new (name: &'static str ) -> Humanoid {
        Humanoid {
            name: name,
            pack: Vec::new(),
        }
    }

    fn name (&self) -> &'static str {
        self.name
    }
}

trait Foo { }

impl<'a> Foo for &'a str { }

impl<'a> Foo for Vec<&'a str> { }

fn main() {
    let s = "hello";
    let v = vec!["foo", "bar"];

    let list = vec![Box::new(s) as Box<Foo>, Box::new(v) as Box<Foo>];
}
