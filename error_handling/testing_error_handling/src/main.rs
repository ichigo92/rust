use std::net::IpAddr;


fn main() {
    let home: IpAddr = "::1".parse().unwrap();

    // let x: u32 = "67".parse().unwrap();

    assert_eq!(home.is_ipv6(), true);
}
