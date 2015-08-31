
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
pub struct GameObject<'a>{
    pub name: &'a str,
    pub value: i32,
}
pub struct Weapon<'a> {
    parent: GameObject<'a>
}

trait GameObjectTrait<'a> {
    fn get_parent<'b>(&'a self) -> &GameObject<'a>;
    //fn get_parent_mut<'a>(&'a mut self) -> &'a mut GameObject;
    //Self may make this unsafe object...
    fn name(&'a self) -> &'a str { "bleh" }
    fn value(self) -> i32;
    fn new (name: &'a str, value: i32) -> GameObject<'a> {
        GameObject {
            name: name,
            value: value,
        }
    }
    fn use_obj<'b>(&self) -> &'b str {
        "Your object does 1 damage."
    }
}

impl<'a,'b> GameObjectTrait<'b> for GameObject<'a> {
    fn get_parent(&'b self) -> &GameObject<'b> { self }
    //fn get_parent_mut<'a>(&'a mut self) -> &'a mut GameObject { &mut self }
    //fn name(&'b self) -> &'b str { self.name }
    fn value(self) -> i32 { self.value }
    fn new(name: &'b str, value: i32 ) -> GameObject<'b> {
         GameObject {
            name:  name,
            value: value,
        }
    }
}

trait WeaponTrait<'a>: GameObjectTrait<'a> {
    fn get_child(&'a self) -> &'a Weapon;
    // Other child methods...
}

impl<'a, 'b> WeaponTrait<'b> for Weapon<'a> {
    fn get_child(&'b self) -> &'b Weapon { self }
}

impl<'a,'b> GameObjectTrait<'b> for Weapon<'a> {
    fn get_parent(&'b self) -> &GameObject<'b> { &self.parent }
    //fn get_parent_mut<'a>(&'a mut self) -> &'a mut GameObject { &mut self.parent }
    //fn name(&'b self) -> &'b str { self.parent.name }
    fn value(self) -> i32 { self.parent.value }

}

impl<'a> Weapon<'a> {
    pub fn new (name: &str, value: i32) -> Weapon {
        Weapon {
            parent: GameObject {
                name: name,
                value: value,
            }
        }
    }
}

/*
pub struct Weapon {
    pub name:           &'static str,
    pub base_value:     i32,
}

impl Weapon {
    pub fn new (name: &'static str, base_value: i32) -> &Weapon {
        &Weapon {
             name:          name,
             base_value:    base_value,
         }
    }

    pub fn value (&self) -> &i32 {
        &(self.base_value * 10)
    }
}

impl GameObject for Weapon {
    pub fn new (name: &'static str) -> &Weapon {
        &Weapon {
             name:          name,
             base_value:    100,
         }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn value (&self) -> &i32 {
        &(self.base_value * 10)
    }
}
*/
