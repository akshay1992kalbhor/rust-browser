use std::io::{Read, Write};
use std::net::TcpStream;

fn f1(url: &str) {
    assert!(url.starts_with("http://"));
    let url_wo_scheme = String::from(url.strip_prefix("http://").unwrap());
    let (host, path) = url_wo_scheme
        .split_once("/")
        .map(|(h, p)| (String::from(h), String::from(p)))
        .unwrap();
    let path = "/".to_owned() + &path;
    println!("Scheme: http, Host: {}, Path: {}", &host, &path);
}

fn f2() -> std::io::Result<usize> {
    let mut stream = TcpStream::connect(("example.org", 80)).unwrap();
    let mut sent = 0;
    sent += stream.write(b"GET /index.html HTTP/1.0\r\n")?;
    sent += stream.write(b"Host: example.org\r\n\r\n")?;
    Ok(sent)
}

fn f3() -> std::io::Result<usize> {
    let mut stream = TcpStream::connect(("example.org", 80)).unwrap();
    let data = b"GET /index.html HTTP/1.0\r\n\
        Host: example.org\r\n\r\n";
    let sent = stream.write(data)?;

    let mut response_string = String::new();
    let _ = stream.read_to_string(&mut response_string);
    let status_line = response_string.lines().nth(0).unwrap();
    match status_line
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .as_slice()
    {
        &[version, status, explanation] => {
            assert!(
                status == "200",
                "Status: {}, Explanation: {}",
                status,
                explanation
            );
        }
        _ => assert!(false),
    }
    Ok(sent)
}

fn main() {
    let data = b"GET /index.html HTTP/1.0\r\n\
        Host: example.org\r\n\r\n";
    println!("{}", std::str::from_utf8(data).unwrap());
}
