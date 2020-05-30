fn main() {
	for_loop();
}

fn loop_test() {
	let mut counter = 0;
    let _val_counter = loop {
    	println!("Hello World");
    	counter = counter + 1;
    	if counter == 3 {
    		break counter	// expression
    	}
    };

    println!("{}", _val_counter);
}


fn while_loop() {
	let mut counter = 0;

	while counter < 3 {
		println!("Hello World");
		counter = counter + 1;
	}

	let mut counter_1 = 0;
	let lottery_number = [1, 2, 3, 4, 5, 6, 7, 8, 9];

	while counter_1 < lottery_number.len() {
		println!("{}", lottery_number[counter_1]);
		counter_1 = counter_1 + 1;
	}
}

fn for_loop() {
	for a in (0..5).rev() {
		println!("{}. Hello World", a);
	}

	let lottery_number = [1, 23, 45, 67, 89];
	for num in lottery_number.iter() {
		println!("{}", num);
	}
}