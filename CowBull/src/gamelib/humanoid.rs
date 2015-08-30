use gamelib::actinator;
use gamelib::object;

pub struct Humanoid {
    pub name: &'static str,
    pub pack: Vec<object::Weapon>,
}

impl Humanoid {
    pub fn new (name: &'static str ) -> Humanoid {
        Humanoid {
            name: name ,
            pack: Vec::new(),
        }
    }

    pub fn add_to_pack (&mut self, w: object::Weapon) {
        self.pack.push(w);
    }
}

impl actinator::Actinator for Humanoid {
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
