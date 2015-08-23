mod actinator;

//beast
pub struct Beast {
    name: &'static str,
}

impl Beast {
    pub fn new (name: &'static str) -> Beast {
        Beast { name: name }
    }

    fn name (&self) -> &'static str {
        self.name
    }

    fn action (&self) {
        println!("Beast {} SMASH!!", self.name);
    }
}

pub struct Humaniod {
    pub name: &'static str
}

impl Humaniod {
    pub fn new (name: &'static str ) -> Humaniod {
        Humaniod { name: name }
    }

    fn name (&self) -> &'static str {
        self.name
    }

    fn action (&self)  {
        println!("Humaniod {} casts spell!", self.name);
    }
}

//monster type -- maybe sub of humaniod....
pub struct Monster {
    name: &'static str,
}
