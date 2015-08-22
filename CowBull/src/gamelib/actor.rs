trait Amphibian {
    fn hop(&self);
}

struct Frog {
    name: &'static str,
}

struct Toad {
    name: &'static str,
}

impl Amphibian for Frog {
    fn hop(&self) {
        println!("Hopping frog {}", self.name);
    }
}

impl Amphibian for Toad {
    fn hop(&self) {
        println!("Hopping toad {}", self.name);
    }
}

pub fn main() {
    let frog = Frog { name: "Frogger" };
    let toad = Toad { name: "Toady" };
    {
        let animals = vec!(&frog as &Amphibian, &toad as &Amphibian);

        for animal in animals.iter() {
            animal.hop();
        }
    }
}
