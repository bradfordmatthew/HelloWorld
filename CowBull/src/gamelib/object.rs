
/*
struct Parent {...}
struct Child { parent: Parent, ... }

trait ParentTrait {
    get_parent<'a>(&'a self) -> &'a Parent;
    // Have all other methods you want for your 'Parent' struct here, and use 'self.get_parent()' if you need to access data members of parent.
}

// Now you only need to impl one method for the Parent struct.
impl ParentTrait for Parent {
    get_parent<'a>(&'a self) -> &'a Parent { self }
}

// Now your child trait could be...
trait ChildTrait: ParentTrait {
    get_child<'a>(&'a self) -> &'a Child;
    // Other child methods...
}

// And now for your child struct you only need to implement get_parent and get_child...
impl ChildTrait for Child {
    fn get_child<'a>(&'a self) -> &'a Child { self }
}
impl ParentTrait for Child {
    fn get_parent<'a>(&'a self) -> &'a Parent { &self.parent }
    get_parent_mut<'a>(&'a mut self) -> &'a mut Parent { &mut self.parent }
}
*/
pub trait GameObject {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn use_obj(&self) -> &'static str {
        "You weapon does 10 damage."
    }

}

pub struct Weapon {
    pub name:           &'static str,
    pub base_value:     i32,
}

impl Weapon {
    pub fn new (name: &'static str, base_value: i32) -> Weapon {
        Weapon {
             name:          name,
             base_value:    base_value,
         }
    }

    pub fn value (&self) -> i32 {
        self.base_value * 10
    }
}

impl GameObject for Weapon {
    fn new (name: &'static str) -> Weapon {
        Weapon {
             name:          name,
             base_value:    100,
         }
    }

    fn name (&self) -> &'static str {
        self.name
    }

}
