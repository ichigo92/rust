use std::thread;
use std::time::Duration;

/*fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}*/

/**
 * @brief      Refactor code using Closures
 * 
 *             We want to define code in one place in
 *             our program, but only execute that code
 *             where we actually need the result.
 *             This is a use case for closures!
 * 
 * @problem    We’re still calling the closure twice in
 * 			   the first if block, which will call the
 * 			   expensive code twice and make the user
 * 			   wait twice as long as they need to.
 *
 * @param      intensity      The intensity
 * @param      random_number  The random number
 */
fn generate_workout(intensity: u32, random_number: u32) {
	/*let expensive_closure = simulated_expensive_calculation(intensity);*/
	// Move the body of simulated_expensive_calculation inside expensive_closure
	// We no longer need the simulated_expensive_calculation(intensity) function
	let expensive_closure = |num| {
		println!("calculating slowly...");
	    thread::sleep(Duration::from_secs(2));
	    num
	};
	
	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			expensive_closure(intensity)
		);
		println!(
			"Next, do {} situps!",
			expensive_closure(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				expensive_closure(intensity)
			);
		}
	}
}

/**
 * @brief      Refactor by extracting the duplicated call
 *             to the simulated_expensive_calculation() function
 *             into a variable expensive_result
 * 
 * @problem    Unfortunately, we’re now calling this function and
 *             waiting for the result in all cases, which includes
 *             the inner if block that doesn’t use the result value at all.
 *
 * @param      intensity      The intensity
 * @param      random_number  The random number
 */
/*fn generate_workout(intensity: u32, random_number: u32) {
	let expensive_result = simulated_expensive_calculation(intensity);
	
	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			expensive_result
		);
		println!(
			"Next, do {} situps!",
			expensive_result
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				expensive_result
			);
		}
	}
}*/

/**
 * @brief      Original Workout Codde, too many calls
 *             for simulated_expensive_calculation() function
 *
 * @param      intensity      The intensity
 * @param      random_number  The random number
 */
/*fn generate_workout(intensity: u32, random_number: u32) {
	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			simulated_expensive_calculation(intensity)
		);
		println!(
			"Next, do {} situps!",
			simulated_expensive_calculation(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				simulated_expensive_calculation(intensity)
			);
		}
	}
}*/

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

