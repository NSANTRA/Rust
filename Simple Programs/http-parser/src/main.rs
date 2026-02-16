use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct HTTPRequest {
    method: String,
    route: String,
    version: String,
    headers: HashMap<String, String>
}

fn parser_as_string(request_bytes: &[u8]) -> Result<HTTPRequest, String> {
    let request_str = std::str::from_utf8(request_bytes).unwrap();

    let mut lines = request_str.split("\r\n");

    let first = lines.next().unwrap();
    let mut first_line = first.split(" ");

    let method: String = first_line.next().unwrap().to_string();
    let route: String = first_line.next().unwrap().to_string();
    let version: String = first_line.next().unwrap().to_string();

    let mut header: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        else {
            let (key, value) = line.split_once(": ").unwrap();
            header.insert(key.to_string(), value.to_string());
        }
    }

    Ok(HTTPRequest {
        method,
        route,
        version,
        headers: header
    })
}

fn main() {
    let sample = "GET /hello.txt HTTP/1.1\r\n\
User-Agent: curl/7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3\r\n\
Host: www.example.com\r\n\
Accept-Language: en, mi\r\n\
\r\n";

    let bytes: &[u8] = sample.as_bytes();
    // println!("{:#?}", bytes)                 // Prettify (print on new line)
    // println!("{:?}", bytes);

    let test = parser_as_string(bytes);

    // println!("{}", test);

    println!("Method: {:?}", test.clone().unwrap().method);
    println!("Route: {:?}", test.clone().unwrap().route);
    println!("Version: {:?}", test.clone().unwrap().version);
    println!("Header: {:?}", test.clone().unwrap().headers);
}