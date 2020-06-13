#[derive(Debug)]
enum IPAddrKind {
	V4,
	V6
}


fn route(_x: IPAddrKind) {
	println!("{:?}", _x);
}



fn main() {
    let _four = IPAddrKind::V4;
    let _six = IPAddrKind::V6;

    route(_four);
    route(_six);
}
