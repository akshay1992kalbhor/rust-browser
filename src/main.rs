use openssl::ssl::{SslConnector, SslMethod};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

fn retstring() -> &'static str {
    "cmon mon"
}

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
struct RequestHeader<'a> {
    host: Option<&'a str>,
    connection: &'a str,
    user_agent: &'a str,
}

fn fun_crap() {
    let data = b"GET /index.html HTTP/1.0\r\n\
        Host: example.org\r\n\r\n";
    //println!("{}", std::str::from_utf8(data).unwrap());
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

struct Browser {}

impl Browser {
    fn new() -> Self {
        Browser {}
    }

    fn load(&self, url: &str) -> String {
        let (body, headers) = self.request(url).unwrap();
        self.show(&body)
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
                //print!("{}", c);
            }
        }
        //println!();
        content
    }

    fn request(&self, url: &str) -> std::io::Result<(String, HashMap<String, String>)> {
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
            //eprintln!("LINE: {:?}", line);
            let (header, value) = line.split_once(":").unwrap();
            headers.insert(header.to_lowercase(), value.trim().to_string());
        }

        let body = what.collect::<String>();

        Ok((body, headers))
    }
}

use druid::piet::{PietTextLayoutBuilder, TextStorage as PietTextStorage};
use druid::text::{Attribute, RichText, TextStorage};
use druid::widget::prelude::*;
use druid::widget::{Button, Controller, Flex, Label, LineBreaking, RadioGroup, RawLabel, Scroll};
use druid::{
    AppLauncher, Color, Data, FontFamily, FontStyle, FontWeight, Lens, LocalizedString,
    TextAlignment, Widget, WidgetExt, WindowDesc,
};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Text Options");

const TEXT: &str = r#"Contrary to what we would like to believe, there is no such thing as a structureless group. Any group of people of whatever nature that comes together for any length of time for any purpose will inevitably structure itself in some fashion. The structure may be flexible; it may vary over time; it may evenly or unevenly distribute tasks, power and resources over the members of the group. But it will be formed regardless of the abilities, personalities,or intentions of the people involved. The very fact that we are individuals, with different talents, predispositions, and backgrounds makes this inevitable. Only if we refused to relate or interact on any basis whatsoever could we approximate structurelessness -- and that is not the nature of a human group.
This means that to strive for a structureless group is as useful, and as deceptive, as to aim at an "objective" news story, "value-free" social science, or a "free" economy. A "laissez faire" group is about as realistic as a "laissez faire" society; the idea becomes a smokescreen for the strong or the lucky to establish unquestioned hegemony over others. This hegemony can be so easily established because the idea of "structurelessness" does not prevent the formation of informal structures, only formal ones. Similarly "laissez faire" philosophy did not prevent the economically powerful from establishing control over wages, prices, and distribution of goods; it only prevented the government from doing so. Thus structurelessness becomes a way of masking power, and within the women's movement is usually most strongly advocated by those who are the most powerful (whether they are conscious of their power or not). As long as the structure of the group is informal, the rules of how decisions are made are known only to a few and awareness of power is limited to those who know the rules. Those who do not know the rules and are not chosen for initiation must remain in confusion, or suffer from paranoid delusions that something is happening of which they are not quite aware."#;

const SPACER_SIZE: f64 = 8.0;

#[derive(Clone, Data, Lens)]
struct AppState {
    text: RichText,
    line_break_mode: LineBreaking,
    alignment: TextAlignment,
}

//NOTE: we implement these traits for our base data (instead of just lensing
//into the RichText object, for the label) so that our label controller can
//have access to the other fields.
impl PietTextStorage for AppState {
    fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl TextStorage for AppState {
    fn add_attributes(&self, builder: PietTextLayoutBuilder, env: &Env) -> PietTextLayoutBuilder {
        self.text.add_attributes(builder, env)
    }
}

/// A controller that updates label properties as required.
struct LabelController;
impl Controller<AppState, RawLabel<AppState>> for LabelController {
    #[allow(clippy::float_cmp)]
    fn update(
        &mut self,
        child: &mut RawLabel<AppState>,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        if old_data.line_break_mode != data.line_break_mode {
            child.set_line_break_mode(data.line_break_mode);
            ctx.request_layout();
        }
        if old_data.alignment != data.alignment {
            child.set_text_alignment(data.alignment);
            ctx.request_layout();
        }
        child.update(ctx, old_data, data, env);
    }
}
pub fn main() {
    // describe the main window

    let browser = Browser::new();
    let what = browser.load("http://example.org/index.html");

    let main_window = WindowDesc::new(|| build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 600.0));
    let text = RichText::new(what.into());
    //.with_attribute(0..9, Attribute::text_color(Color::rgb(1.0, 0.2, 0.1)))
    //.with_attribute(0..9, Attribute::size(24.0))
    //.with_attribute(0..9, Attribute::font_family(FontFamily::SERIF))
    //.with_attribute(194..239, Attribute::weight(FontWeight::BOLD))
    //.with_attribute(764.., Attribute::size(12.0))
    //.with_attribute(764.., Attribute::style(FontStyle::Italic));
    // create the initial app state
    let initial_state = AppState {
        line_break_mode: LineBreaking::Clip,
        alignment: Default::default(),
        text,
    };
    // start the application
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}
fn build_root_widget() -> impl Widget<AppState> {
    let label = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::BLACK)
            .controller(LabelController)
            .background(Color::WHITE)
            .expand_width()
            .padding((SPACER_SIZE * 4.0, SPACER_SIZE))
            .background(Color::grey8(222)),
    )
    .vertical();
    let line_break_chooser = Flex::column()
        .with_child(Label::new("Line break mode"))
        .with_spacer(SPACER_SIZE)
        .with_child(RadioGroup::new(vec![
            ("Clip", LineBreaking::Clip),
            ("Wrap", LineBreaking::WordWrap),
            ("Overflow", LineBreaking::Overflow),
        ]))
        .lens(AppState::line_break_mode);
    let alignment_picker = Flex::column()
        .with_child(Label::new("Justification"))
        .with_spacer(SPACER_SIZE)
        .with_child(RadioGroup::new(vec![
            ("Start", TextAlignment::Start),
            ("End", TextAlignment::End),
            ("Center", TextAlignment::Center),
            ("Justified", TextAlignment::Justified),
        ]))
        .lens(AppState::alignment);
    let button = Button::new("Go!")
        .on_click(|ctx: &mut EventCtx, data: &mut AppState, _env: &Env| println!("AA"));
    let controls = Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_child(alignment_picker)
        .with_spacer(SPACER_SIZE)
        .with_child(line_break_chooser)
        .with_child(button)
        .padding(SPACER_SIZE);
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_child(controls)
        .with_flex_child(label, 1.0)
}
