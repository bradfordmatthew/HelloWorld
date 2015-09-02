use gamelib::object;

pub struct Weapon<'a> {
    parent: object::GameObject<'a>
}

trait WeaponTrait<'a>: object::GameObjectTrait<'a> {
    fn get_child(&'a self) -> &'a Weapon;
    // Other child methods...
}

impl<'a, 'b> WeaponTrait<'b> for Weapon<'a> {
    fn get_child(&'b self) -> &'b Weapon { self }
}

impl<'a,'b> object::GameObjectTrait<'b> for Weapon<'a> {
    fn get_parent(&'b self) -> &object::GameObject<'b> { &self.parent }
    fn value(&'b self) -> i32 { self.parent.value }
}

impl<'a> Weapon<'a> {
    pub fn new (name: &str, value: i32) -> Weapon {
        Weapon {
            parent: object::GameObject {
                name: name,
                value: value,
            }
        }
    }

    //fn get_parent_mut(&'a mut self) -> &'a mut object::GameObject {
    //    &mut self.parent
    //}

    //pub fn name(&'a self) -> &'a str {
    //    self.parent.name
    //}
}

#[test]
fn test_Weapon_new () {
    let weap = Weapon::new("weap1", 1000);
    assert!(weap.parent.name == "weap1" && weap.parent.value == 1000);
}
