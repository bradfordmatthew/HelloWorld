
//hunt2015
pub struct CowBull {
    bull:   bool,
    cow:    bool,
    index:  usize,
    value:  char,
}

impl CowBull {
    pub fn new() ->  CowBull {
        CowBull {
            bull:   false,
            cow:    false,
            index:  0,
            value:  '0',
        }
    }

    pub fn set_bull (&mut self, index: usize, value: char) {
        self.cow = false;
        self.bull = true;
        self.index = index;
        self.value = value;
    }

    pub fn set_cow (&mut self) {
        if self.bull == false {
            self.cow = true;
        }
    }
}

pub struct TheHerd {
    pub herd:   Vec<CowBull>,
}

impl TheHerd {
    pub fn new() -> TheHerd {
        TheHerd {
            herd:   vec!{CowBull::new(), CowBull::new(), CowBull::new(), CowBull::new()},
        }
    }

    pub fn cow_count (&self) -> i32 {
        let mut count = 0;
        for bovine in self.herd.iter() {
            if bovine.cow {
                count = count + 1;
            }
        }
        count
    }

    pub fn bull_count (&self) -> i32 {
        let mut count = 0;
        for bovine in self.herd.iter() {
            if bovine.bull {
                count = count + 1;
            }
        }
        count
    }
}

#[test]
pub fn test_works () {
    let mut cow = CowBull::new();
    assert!(cow.bull == false && cow.cow == false && cow.index == 0 && cow.value == '0');
}

#[test]
pub fn test_cow_update () {
    let mut cow = CowBull::new();
    cow.set_bull(0, '0');
    assert!(cow.bull);
}

#[test]
pub fn test_herd () {
    let herd = TheHerd::new();
    assert!(herd.herd.len() == 4);
}

#[test]
pub fn test_herd_cow_count () {
    let mut herd = TheHerd::new();
    herd.herd[0].set_cow();
    assert!(herd.cow_count() == 1);
}

#[test]
pub fn test_herd_bull_count () {
    let mut herd = TheHerd::new();
    herd.herd[3].set_bull(3, '4');
    assert!(herd.bull_count() == 1);
}
