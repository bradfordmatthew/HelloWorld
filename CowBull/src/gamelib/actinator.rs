use std::borrow::Cow;
//trait seem nice, but I cannot seem to get them to work from a library. Methods from a trait
//are never in scope when delcared in library...
pub trait Actinator<'a> {
    //woo we can make things!
    fn new(name: &'a str) -> Self;

    fn name(&self) -> &'a str;

    fn action(&self)-> Cow<'a, str> {
        let some_string: String = format!("Default actinator action for {}.", self.name()).to_string();
        return Cow::Owned(some_string);
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
