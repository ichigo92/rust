// Method 1
// Create a seperate struct
// Add a field in the struct of type enum
#[derive(Debug)]
enum IPAddrKind {
	V4,
	V6
}

#[derive(Debug)]
struct IPAddress {
	kind: IPAddrKind,
	address: String
}


fn main() {
    let home = IPAddress {
    	kind: IPAddrKind::V4,
    	address: String::from("127.0.0.1")
    };

    println!("{:#?}", home);

    let loopback = IPAddress {
    	kind: IPAddrKind::V6,
    	address: String::from("::1")
    };

    println!("{:#?}", loopback);
}
