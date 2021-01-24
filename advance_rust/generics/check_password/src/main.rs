fn main() {
    check_password(1678);
    check_password(7813);
    let _password = String::from("Karachi");
    string_check_password(_password);
}

fn check_password<T: Display>(x: T) {
	println!("{}", x);
}