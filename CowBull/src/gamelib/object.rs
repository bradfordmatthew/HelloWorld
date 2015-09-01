
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

pub trait GameObjectTrait<'a> {
    fn get_parent<'b>(&'a self) -> &GameObject<'a>;
    fn name(&'a self) -> &'a str { "bleh" }
    fn value(&'a self) -> i32;
    fn use_obj<'b>(&'a self) -> &'b str {
        "Your object does 1 damage."
    }
}

impl<'a,'b> GameObjectTrait<'b> for GameObject<'a> {
    fn get_parent(&'b self) -> &GameObject<'b> { self }
    //fn get_parent_mut<'a>(&'a mut self) -> &'a mut GameObject { &mut self }
    //fn name(&'b self) -> &'b str { self.name }
    fn value(&'b self) -> i32 { self.value }
}





/*

trait Foo {}

struct MyFoo;

impl Foo for MyFoo {}

struct Bar<'a> {
    foo: Box<Foo + 'a>,
}

impl<'a> Bar<'a> {
    fn new(the_foo: Box<Foo + 'a>) -> Bar<'a> {
        Bar { foo: the_foo }
    }

    fn get_foo(&'a self) -> &'a Foo {
        &*self.foo
    }
}

fn main() {
    let mybar = Bar::new(box MyFoo as Box<Foo>);
}

---------------------------------------------------

trait Foo {}

struct MyFoo;

impl Foo for MyFoo {}

struct Bar<'a> {
    foo: &'a (Foo + 'a),
}

impl<'a> Bar<'a> {
    fn new(the_foo: &'a Foo) -> Bar<'a> {
        Bar { foo: the_foo }
    }

    fn get_foo(&'a self) -> &'a Foo {
        self.foo
    }
}

fn main() {
    let myfoo = MyFoo;
    let mybar = Bar::new(&myfoo as &Foo);
}
---------------------------------------------
trait Foo { }

impl<'a> Foo for &'a str { }

impl<'a> Foo for Vec<&'a str> { }

fn main() {
    let s = "hello";
    let v = vec!["foo", "bar"];

    let list = vec![Box::new(s) as Box<Foo>, Box::new(v) as Box<Foo>];
}

--------------------------------------------

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
