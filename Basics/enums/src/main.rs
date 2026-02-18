enum IpVersion {
    V4,
    V6,
    V12(String),
}

fn main() {
    // let four = IpVersion::V4;
    let four = IpVersion::V6;
    let six = IpVersion::V6;
    let custom = IpVersion::V12(String::from("Hello"));

    if let IpVersion::V4 = four {
        println!("IP V4");
    } else if let IpVersion::V6 = six {
        println!("IP V6");
    } else if let IpVersion::V12(_string) = custom {
        println!("IP V12");
    } else {
        println!("IP Invalid");
    }
}