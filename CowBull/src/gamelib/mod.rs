pub mod actinator; //bah missing pub modification on a module! So much grief...oh how we learn.
pub mod beast;
pub mod object;

use std::io;


pub struct Humaniod {
    pub name: &'static str,
    pub pack: Vec<object::Weapon>,
}

impl Humaniod {
    fn new (name: &'static str ) -> Humaniod {
        Humaniod {
            name: name ,
            pack: Vec::new(),
        }
    }

    pub fn add_to_pack (&mut self, w: object::Weapon) {
        self.pack.push(w);
    }
}

impl actinator::Actinator for Humaniod {
    fn new (name: &'static str ) -> Humaniod {
        Humaniod {
            name: name,
            pack: Vec::new(),
        }
    }

    fn name (&self) -> &'static str {
        self.name
    }
}
