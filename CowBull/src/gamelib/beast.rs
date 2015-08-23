use gamelib::actinator;
//beast
pub struct Beast {
    name: &'static str,
}

impl Beast {
    pub fn new (name: &'static str) -> Beast {
        Beast { name: name }
    }
}

impl actinator::Actinator for Beast {
    fn new (name: &'static str) -> Beast {
        Beast { name: name }
    }

    fn name (&self) -> &'static str {
        self.name
    }

    fn action (&self) -> &'static str {
        "Beast Does it's own thing!"
    }
}
