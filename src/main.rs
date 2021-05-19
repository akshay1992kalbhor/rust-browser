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

    assert!(url.starts_with("http://") || url.starts_with("https://"));
    let (scheme, rest) = url.split_once("://").unwrap();
    let (host, path) = rest.split_once("/").unwrap();
    let path = "/".to_owned() + &path;
    (scheme, url, url)
}

fn test_f2() -> std::io::Result<usize> {
    let mut stream = TcpStream::connect(("example.org", 80)).unwrap();
    let mut sent = 0;
    sent += stream.write(b"GET /index.html HTTP/1.0\r\n")?;
    sent += stream.write(b"Host: example.org\r\n\r\n")?;
    Ok(sent)
}

struct RequestHeader<'a> {
    host: Option<&'a str>,
    connection: &'a str,
    user_agent: &'a str,
}

fn lex(body: &str) -> String {
    let mut in_angle = true;
    let mut in_body = false;
    let mut tag = String::new();
    let mut content = String::new();
    for c in body.chars() {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            if tag == "body" {
                in_body = !in_body;
            }
            in_angle = false;
            tag.clear();
        } else if in_angle {
            if c != '/' {
                tag.push(c);
            }
        } else if !in_angle && in_body {
            content.push(c);
        }
    }
    content
}

fn layout(text: String) -> Vec<(u32, u32, char)> {
    Vec::new()
}

struct Browser {
    //display_list: Vec<(u32, u32, char)>,
    title: String,
}

impl Browser {
    fn new() -> Self {
        Browser {
            //display_list: Vec::new(),
            title: String::new(),
        }
    }

    fn render(&self) {}

    fn load(&self, url: &str) -> String {
        let (body, headers) = self.request(url).unwrap();
        lex(&body)
    }

    fn show(&self, body: &str) -> String {
        let mut in_angle = true;
        let mut in_body = false;
        let mut tag = String::new();
        let mut content = String::new();
        for c in body.chars() {
            if c == '<' {
                in_angle = true;
            } else if c == '>' {
                if tag == "body" {
                    in_body = !in_body;
                }
                in_angle = false;
                tag.clear();
            } else if in_angle {
                if c != '/' {
                    tag.push(c);
                }
            } else if !in_angle && in_body {
                content.push(c);
            }
        }
        content
    }

    fn request(&self, url: &str) -> std::io::Result<(String, HashMap<String, String>)> {
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

        let path = "/".to_owned() + &path;
        println!("Scheme: {}, Host: {}, Path: {}", scheme, host, path);
        let req_headers: HashMap<&str, &str> = vec![
            ("Host", host),
            ("Connection", "close"),
            ("User-Agent", "Quantum"),
        ]
        .into_iter()
        .collect();
        let req_headers1 = vec![
            ("Host", host),
            ("Connection", "close"),
            ("User-Agent", "Quantum"),
        ];

        let mut second = format!("GET {} HTTP/1.1\r\n", path);
        second.extend(
            req_headers1
                .iter()
                .map(|(k, v)| k.to_string() + ": " + v + "\r\n"),
        );
        second.push_str("\r\n");

        /* INIT AND SEND REQUEST */
        let first = format!(
            "GET {} HTTP/1.1\r\n\
        Host: {}\r\n
        Connection: close\r\n
        User-Agent: Quantum\r\n\r\n",
            path, host
        );

        /* Connection: close\r\n
        User-Agent: Mozilla/5.0\r\n */
        let mut response_string = String::new();
        let mut stream = TcpStream::connect((host, port)).unwrap();

        if scheme == "https" {
            let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
            let mut stream = connector.connect(host, &stream).unwrap();

            let sent = stream.write(second.as_bytes())?;
            let _ = stream.read_to_string(&mut response_string);
        } else {
            let sent = stream.write(second.as_bytes())?;
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
            let (header, value) = line.split_once(":").unwrap();
            headers.insert(header.to_lowercase(), value.trim().to_string());
        }
        let body = what.collect::<String>();
        Ok((body, headers))
    }

    fn setup_window_and_launch_app(&self) {
        let mut file_menu = Menu::new();
        file_menu.add_item(
            0x100,
            "E&xit",
            Some(&HotKey::new(SysMods::Cmd, "q")),
            true,
            false,
        );
        file_menu.add_item(
            0x101,
            "O&pen",
            Some(&HotKey::new(SysMods::Cmd, "o")),
            true,
            false,
        );
        let mut menubar = Menu::new();
        menubar.add_dropdown(Menu::new(), "Application", true);
        menubar.add_dropdown(file_menu, "&File", true);
        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_title("MY_TITLE");
        builder.set_menu(menubar);
        //builder.set_handler(Box::new(HelloState::default()));

        match builder.build() {
            Ok(window) => window.show(),
            Err(e) => eprintln!("ERROR: {}", e),
        }

        app.run(None);
    }
}

use druid::kurbo::Size;
use druid::piet::{Color, RenderContext};
use druid_shell::{Application, HotKey, Menu, SysMods, WindowBuilder, WindowHandle};

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0xf0, 0xea);

use druid::widget::Label;
use druid::widget::Painter;
use druid::widget::Widget;
fn ui_builder() -> impl Widget<String> {
    let my_painter = Painter::new(|ctx, w: &String, _| {});
    let label = Label::new(|data: &String, _env: &_| format!("Default: {}", data));
    label
}

#[derive(Default)]
struct MyWidget {
    text: String,
}

impl Widget<String> for MyWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut String,
        env: &druid::Env,
    ) {
        println!("EVENT");
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &String,
        env: &druid::Env,
    ) {
        //todo!()
        println!("LIFECYCLE");
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &String,
        data: &String,
        env: &druid::Env,
    ) {
        println!("UPDATE");
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &String,
        env: &druid::Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &String, env: &druid::Env) {
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::RED);
        match ctx.text().new_text_layout(data.clone()).build() {
            Ok(layout) => ctx.draw_text(&layout, (0.0, 0.0)),
            Err(_) => (),
        }
    }
}

use druid::piet::Text;
use druid::piet::TextLayoutBuilder;

fn make_menu_bar() -> Menu {
    let mut file_menu = Menu::new();
    file_menu.add_item(
        0x100,
        "E&xit",
        Some(&HotKey::new(SysMods::Cmd, "q")),
        true,
        false,
    );
    file_menu.add_item(
        0x101,
        "O&pen",
        Some(&HotKey::new(SysMods::Cmd, "o")),
        true,
        false,
    );

    let mut menubar = Menu::new();
    menubar.add_dropdown(Menu::new(), "Application", true);
    menubar.add_dropdown(file_menu, "&File", true);
    menubar
}
use druid::{AppLauncher, WindowDesc};
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert!(args.len() > 1);

    let url = &args[1];
    let browser = Browser::new();
    let body = browser.load(url);

    println!("BODY: {}", body);
    let main_window = WindowDesc::new(|| MyWidget {
        text: String::new(),
    });

    let data = String::from("Hoorway");
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(body);
}

fn druid_shell_init() {
    //let app = Application::new().unwrap();
    //let mut builder = WindowBuilder::new(app.clone());
    //builder.set_title("MY_TITLE");
    //builder.set_menu(make_menu_bar());
    //builder.set_handler(Box::new(HelloState::default()));

    //match builder.build() {
    //    Ok(window) => window.show(),
    //    Err(e) => eprintln!("ERROR: {}", e),
    //}
}
