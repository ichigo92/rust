fn main() {
    let result = gives_ownership();
    println!("{}", result);

    let _s = String::from("Pakistan");
    // let result1 = takes_and_gives_back(_s);

    let (result1, result2) = length(_s);
    println!("The length of the word {} is {}", result1, result2);
}


fn gives_ownership () -> String {
	let s = String::from("Hello World");
	s
}

fn takes_and_gives_back(x: String) -> String {
	x
}

fn length(name: String) -> (usize, String) {
	(name.len(), name)
}
