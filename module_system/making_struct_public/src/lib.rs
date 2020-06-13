mod back_of_house {
	
	#[derive(Debug)]
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String
	}

	impl Breakfast {
		pub fn new (toast: String) -> Breakfast {
			Breakfast {
				toast,
				seasonal_fruit: String::from("peaches")
			}
		}
	}
}

fn eat_at_restaurant() {
	let mut meal = back_of_house::Breakfast::new(String::from("wheat"));
	println!("{:#?}", meal);
	meal.toast = String::from("barley");
	println!("{}", meal.toast);

	// field `seasonal_fruit` of struct `back_of_house::Breakfast` is private
	// meal.seasonal_fruit = String::from("oranges");
}