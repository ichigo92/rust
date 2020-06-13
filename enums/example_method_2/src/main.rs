fn main() {
    // let _home = IPAddress::V4(String::from("127.0.0.1"));
    // let _loopback = IPAddress::V6(String::from("::1"));
    
    let _home = IPAddress::V4(127,0,0,1); 
    let _loopback = IPAddress::V6(String::from("::1"));

    println!("{:#?}", _home);
    println!("{:#?}", _loopback);
}

// #[derive(Debug)]
// enum IPAddress {
// 	V4(String),
// 	V6(String)
// }


#[derive(Debug)]
enum IPAddress {
	V4(u8, u8, u8, u8),
	V6(String)
}