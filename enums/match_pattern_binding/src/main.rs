#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska
}

fn value_in_cents(coin: Coin) -> u8 {
	
	match coin {
		Coin::Penny => {
			println!("Lucky Penny");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("{:?}", state);
			25
		}
			
	}



	// match coin {
	// 	Coin::Penny => 1,
	// 	Coin::Nickel => 5,
	// 	Coin::Dime => 10,
	// 	Coin::Quarter => 25
	// }
}

fn main() {
    let _quarter = Coin::Quarter(UsState::Alaska);
    let cents = value_in_cents(_quarter);
    println!("{:?}", cents);
}
