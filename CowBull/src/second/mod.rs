use gamelib;
use std::mem;
//Print for any actinator. Everything that is not enviroment or object will be actinator
//and have some sort of action and name in common.
fn print_actinator<T: gamelib::actinator::Actinator>(t: &T) {
    println!("{}, by: {}",t.action(), t.name());
}

//maybe make another game loop in here or something
pub fn game_without_a_name () -> bool {
    let mut     human2: gamelib::humanoid::Humanoid = gamelib::actinator::Actinator::new("test doode!");
    let         human1                              = gamelib::humanoid::Humanoid::new("My names booger!");
    let         beast:  gamelib::beast::Beast       = gamelib::actinator::Actinator::new("The Beast!");
    let         weap1                               = gamelib::object::Weapon::new("The Blade of blades", 28);
    //let         weap2: &gamelib::object::GameObject = &gamelib::object::GameObject::new("weapon 2", 1000);
    //how about a vec dependent on the actinator implementation of the class...
    //vec!{&beast as &gamelib::actinator::Actinator, &human2 as &gamelib::actinator::Actinator};
    //using as to cast from one struct type to another did not work...will find another way to do this...

    unsafe {
        human2.add_to_pack(mem::transmute::<&gamelib::object::Weapon,&gamelib::object::GameObject>(&weap1));
    }

    //human2.add_to_pack(weap2);

    //let mon_vec = vec!
    //rintln!("weap2 {}", weap2.name);
    print_actinator(&human2);
    print_actinator(&beast);
    print_actinator(&human1);
    println!("object name {} and value ", human2.pack[0].name);
    //println!("Object name {} and value {}.", weap2.name, weap2.value());

    true
}
