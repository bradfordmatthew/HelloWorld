pub mod actinator; //bah missing pub modification on a module! So much grief...oh how we learn.
pub mod beast;


pub struct Humaniod {
    pub name: &'static str
}

impl Humaniod {
    pub fn new (name: &'static str ) -> Humaniod {
        Humaniod { name: name }
    }
}

impl actinator::Actinator for Humaniod {
    fn new (name: &'static str ) -> Humaniod {
        Humaniod { name: name }
    }

    fn name (&self) -> &'static str {
        self.name
    }
}
