//pub mod world;
//As this struct will exists for the duration of the application it will be 'static
pub struct World;// {
    //gameobj_count:      &'static mut i64,
    //actinator_count:    &'static mut i64,
//}

impl World {
    pub fn draw_world() {
        println!("-----------------");
        println!("|...............|");
        println!("|...............|");
        println!("|...............|");
        println!("|...............|");
        println!("-----------------");
    }
}
