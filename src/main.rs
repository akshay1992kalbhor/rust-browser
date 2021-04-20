use openssl::ssl::{SslConnector, SslMethod};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

fn parse_url(url: &str) -> (&str, &str, &str) {
    /* http://www.example.org/index.html
     *
     * http:// -> scheme
     * www.example.org -> host
     * /index.html -> path
     *
     */
    let cs = ["http", "https"];
    // Unique operator
    let cs1 = &cs[..];

    //url.starts_with(&cs[..]);

    assert!(url.starts_with("http://") || url.starts_with("https://"));
    let (scheme, rest) = url.split_once("://").unwrap();
    let (host, path) = rest.split_once("/").unwrap();
    let path = "/".to_owned() + &path;
    //println!("Scheme: {}, Host: {}, Path: {}", scheme, host, &path);
    (scheme, url, url)
}

fn test_f2() -> std::io::Result<usize> {
    let mut stream = TcpStream::connect(("example.org", 80)).unwrap();
    let mut sent = 0;
    sent += stream.write(b"GET /index.html HTTP/1.0\r\n")?;
    sent += stream.write(b"Host: example.org\r\n\r\n")?;
    Ok(sent)
}

fn request(url: &str) -> std::io::Result<(String, HashMap<String, String>)> {
    /*
     *
     *
     *
     *
     *
     */

    /* FIGURE OUT PARAMS */
    assert!(url.starts_with("http://") || url.starts_with("https://"));
    let (scheme, url) = url.split_once("://").unwrap();
    let mut port: u16 = match scheme {
        "http" => 80,
        "https" => 443,
        _ => unreachable!(),
    };

    let (mut host, path) = url.split_once("/").unwrap();
    if host.contains(":") {
        let (h, p) = host.split_once(":").unwrap();
        port = p.parse::<u16>().unwrap();
        host = h;
    }

    //let p = &url[idx..];
    //url.get(0..2);

    let path = "/".to_owned() + &path;
    println!("Scheme: {}, Host: {}, Path: {}", scheme, host, path);

    /* INIT AND SEND REQUEST */
    let first = format!(
        "GET {} HTTP/1.0\r\n\
        Host: {}\r\n\r\n",
        path, host
    );
    let mut response_string = String::new();
    let mut stream = TcpStream::connect((host, port)).unwrap();

    if scheme == "https" {
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let mut stream = connector.connect(host, &stream).unwrap();

        let sent = stream.write(first.as_bytes())?;
        let _ = stream.read_to_string(&mut response_string);
    } else {
        let sent = stream.write(first.as_bytes())?;
        let _ = stream.read_to_string(&mut response_string);
    }

    // Look for a better pattern
    let newlines_found = response_string
        .find('\n')
        .expect("read_to_string removed all newlines");

    println!("NL: {}", newlines_found);
    let mut what = response_string.lines();
    let status_line = what.next().unwrap();

    /* STATUS */
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

    /* HEADERS */
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let line = what.next().unwrap();
        if line == "" {
            break;
        }
        //eprintln!("LINE: {:?}", line);
        let (header, value) = line.split_once(":").unwrap();
        headers.insert(header.to_lowercase(), value.trim().to_string());
    }

    let body = what.collect::<String>();

    Ok((body, headers))
}

fn show(body: &str) {
    let mut in_angle = true;
    for c in body.chars() {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            in_angle = false;
        } else if !in_angle {
            print!("{}", c);
        }
    }
    println!();
}

fn load(url: &str) {
    let (body, headers) = request(url).unwrap();
    show(&body);
}

fn fun_crap() {
    let data = b"GET /index.html HTTP/1.0\r\n\
        Host: example.org\r\n\r\n";
    //println!("{}", std::str::from_utf8(data).unwrap());
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert!(args.len() > 1);
    load(&args[1]);
}
