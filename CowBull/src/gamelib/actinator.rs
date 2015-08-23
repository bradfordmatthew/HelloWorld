//trait seem nice, but I cannot seem to get them to work from a library. Methods from a trait
//are never in scope when delcared in library...
pub trait Actinator {
    //woo we can make things!
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn action(&self)-> &'static str {
        "default stuff from actinator"
    }
    /*
    fn move(&self);
    fn attack (&self);
    fn get (&self);
    fn sleep (&self);
    fn eat (&self);
    fn die (&self);
    */
}
//maybe mod.rs will contain all the tests for other code files in gamelib? Maybe
//tests will be contained in the classes themselves? structure....
//maybe mod.rs contains factory methods?
//humaniod --intelligentish type
