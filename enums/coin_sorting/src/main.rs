#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
	
	match coin {
		Coin::Penny => {
			println!("Lucky Penny");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25
	}



	// match coin {
	// 	Coin::Penny => 1,
	// 	Coin::Nickel => 5,
	// 	Coin::Dime => 10,
	// 	Coin::Quarter => 25
	// }
}

fn main() {
    let dime = Coin::Dime;
    let cents = value_in_cents(dime);
    println!("{:?}", cents);

    let penny = Coin::Penny;
    let cents = value_in_cents(penny);
    println!("{:?}", cents);
}
