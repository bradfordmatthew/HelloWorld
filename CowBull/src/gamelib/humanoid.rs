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
