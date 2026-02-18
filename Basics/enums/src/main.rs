enum IpVersion {
    V4,
    V6,
    V12(String),
}

fn main() {
    let four = IpVersion::V4;
    let six = IpVersion::V6;
    let custom = IpVersion::V12;

    match four {
        IpVersion::V4 => {
            println!("IP version V4");
        },
        IpVersion::V6 => {
            println!("IP version V6");
        },
        _ => {
            println!("IP version Other");
        }
    }

    match six {
        IpVersion::V4 => {
            println!("IP version V4");
        },
        IpVersion::V6 => {
            println!("IP version V6");
        },
        _ => {
            println!("IP version Other");
        }
    }

    if let IpVersion::V4 = four {
        println!("IP version V4");
    }
}