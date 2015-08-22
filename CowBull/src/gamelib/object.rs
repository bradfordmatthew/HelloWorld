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
