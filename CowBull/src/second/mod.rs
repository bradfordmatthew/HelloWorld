use gamelib;
use worldlib;

//use std::boxed;
//Print for any actinator. Everything that is not enviroment or object will be actinator
//and have some sort of action and name in common.
fn print_actinator<T: gamelib::actinator::Actinator<'static>>(t: &T) {
    println!("{}, by: {}",t.action(), t.name());
}

//maybe make another game loop in here or something
pub fn game_without_a_name () -> bool {
    let mut     human2: gamelib::humanoid::Humanoid = gamelib::actinator::Actinator::new("test doode!");
    let         human1                              = gamelib::humanoid::Humanoid::new("My names booger!");
    let         beast:  gamelib::beast::Beast       = gamelib::actinator::Actinator::new("The Beast!");
    let         weap1                               = gamelib::weapon::Weapon::new("The Blade of blades", 28);
    let         weap2: gamelib::weapon::Weapon      = gamelib::weapon::Weapon::new("weapon 2", 1000);
    //how about a vec dependent on the actinator implementation of the class...
    //vec!{&beast as &gamelib::actinator::Actinator, &human2 as &gamelib::actinator::Actinator};
    //using as to cast from one struct type to another did not work...will find another way to do this...
    /*
    unsafe {
        human2.add_to_pack(mem::transmute::<&gamelib::object::Weapon,&gamelib::object::GameObject>(&weap1));
    }vec![Box::new(s) as Box<Foo>, Box::new(v) as Box<Foo>];
    */
    human2.add_to_pack(Box::new(weap1) as Box<gamelib::object::GameObjectTrait>);
    human2.add_to_pack(Box::new(weap2) as Box<gamelib::object::GameObjectTrait>);
    //human2.add_to_pack(weap2);

    //let mon_vec = vec!
    //rintln!("weap2 {}", weap2.name);
    print_actinator(&human2);
    print_actinator(&beast);
    print_actinator(&human1);

    /*
    for obj.unwrap() in human2.pack.iter() {
        println!("object name {} and value ", obj.name);
    }
    */
    println!("{} has {} items in thier pack.", human2.name, human2.pack.len());
    worldlib::world::World::draw_world();

    //let obj = human2.remove_from_pack().name();

    //println!("removed {} from pack", obj);
    //println!("Object name {} and value {}.", weap2.name, weap2.value());

    true
}
