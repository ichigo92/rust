fn main() {
    let _y = 10;		// statement

    // statement performs an action and never returns a value
    // expression will always returns a value

    // let z = (let u = 9);

    let number = {
    	let o = 19;
    	o + 1		// expression, because its returning a value
    };		// statement, its performing an action
    		// action is its binding a value to a variable

    println!("{}", number);

}
