use gamelib::actinator;
use gamelib::object;

pub struct Humanoid<'a> {
    pub name: &'a str,
    pub pack: Vec<Box<object::GameObjectTrait<'a>>>,
}

impl<'a> Humanoid<'a> {
    pub fn new (name: &'a str ) -> Humanoid<'a> {
        Humanoid {
            name: name,
            pack: Vec::new(),
        }
    }

    pub fn add_to_pack (&mut self, w: Box<object::GameObjectTrait<'a>>) {
        self.pack.push(w);
    }
/*
    pub fn remove_from_pack (&self) -> Box<object::GameObjectTrait<'a>>{
        self.pack[0]
    }
*/
}

impl<'a> actinator::Actinator<'a> for Humanoid<'a> {
    fn new (name: &'a str ) -> Humanoid<'a> {
        Humanoid {
            name: name,
            pack: Vec::new(),
        }
    }

    fn name (&self) -> &'a str {
        self.name
    }
}
