
//hunt2015
pub struct cow_bull {
    bull:   bool,
    cow:    bool,
    index:  i32,
    value:  i32,
}

impl cow_bull {
    pub fn new() ->  cow_bull {
        cow_bull {
            bull:   false,
            cow:    false,
            index:  0,
            value:  0,
        }
    }

    pub fn set_bull (&mut self) {
        self.cow = false;
        self.bull = true;
    }
}

#[test]
pub fn test_works () {
    let mut cow = cow_bull::new();
    assert!(cow.bull == false && cow.cow == false && cow.index == 0 && cow.value == 0);
}

#[test]
pub fn test_cow_update () {
    let mut cow = cow_bull::new();
    cow.set_bull();
    assert!(cow.bull);
}
