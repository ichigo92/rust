#[derive(Debug)]
struct Superman {
	name: String
}

struct Batman {
	name: String
}

struct Hulk {
	name: String
}

struct Spiderman {
	name: String
}

pub trait Power {
	// fn power_score(&self) -> u8;
	// Default implementation
	fn power_score(&self) -> u8 {
		50
	}
}


impl Power for Superman {
	fn power_score(&self) -> u8 {
		100
	}
}

impl Power for Batman {
	fn power_score(&self) -> u8 {
		80
	}
}


// Both Characters will use the default implementation

impl Power for Hulk {}
impl Power for Spiderman {}


fn main() {
    let _superman = Superman {
    	name: String::from("Superman")
    };

    let _batman = Batman {
    	name: String::from("Batman")
    };

    let _hulk = Hulk {
    	name: String::from("Hulk")
    };

    let _spiderman = Spiderman {
    	name: String::from("Spiderman")
    };

    println!("{:?}", _superman.power_score());
    println!("{:?}", _batman.power_score());
    println!("{:?}", _hulk.power_score());
    println!("{:?}", _spiderman.power_score());
}
