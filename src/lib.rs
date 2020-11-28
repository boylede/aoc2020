use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue}};

use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};

const YEAR: i32 = 2020;

pub mod day1;

pub const DAYS: &[Day] = &[
    Day {index: 1, runner: day1::run},
];

/// Wrap the day's runner function so we can store all loaded days in a vec
#[derive(Debug, Clone)]
pub struct Day {
    runner: fn(File),
    pub index: i32,
}

impl Day {
    pub fn new(index: i32, runner: fn(File)) -> Self {
        Day { runner, index }
    }
    pub fn run(self: &Self, input: File) {
        (self.runner)(input);
    }
    pub fn cache_input_and_run(&self, session: &Session) {
        let file = cache_files(self.index, &session);
        if let Ok(input) = file {
            (self.runner)(input);
        } else {
            println!("Couldn't get input file for day {}, quitting.", self.index);
        }
    }
    pub fn run_with_cached_input(&self) {
        let file_path = input_cache_path(self.index);
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&file_path);
        match file {
            Ok(input) => self.run(input),
            Err(_) => println!("No cached input available"),
        }
    }
    pub fn run_with_test_input(&self, input_filename: &str) {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(input_filename);
        if let Ok(input ) = file {
            (self.runner)(input);
        } else {
            println!("couldn't open test input file {}", input_filename);
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Day #{}", self.index)
    }
}

pub fn cache_files(day: i32, session: &Session) -> Result<File, std::io::Error> {
    cache_instructions_for_day(day, &session)?;
    cache_input_for_day(day, &session)
}

pub fn input_cache_path(day: i32) -> String {
    format!("input/day{}.txt", day)
}
pub fn input_url(day: i32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", YEAR, day)
}
pub fn instruction_cache_path(day: i32) -> String {
    format!("instructions/day{}.md", day)
}
pub fn instruction_cache_url(day: i32) -> String {
    format!("https://adventofcode.com/{}/day/{}", YEAR, day)
}

pub fn cache_input_for_day(day: i32, session: &Session) -> Result<File, std::io::Error> {
    let file_path = input_cache_path(day);
    let file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_path);
    let url = input_url(day);
    if let Err(_e) = file {
        println!("Downloading inputs for this day.");
        session.download_file(&url, &file_path)
    } else {
        file
    }
}

pub fn cache_instructions_for_day(day: i32, session: &Session) -> Result<(), std::io::Error> {
    let file_path = instruction_cache_path(day);
    let file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_path);

    if let Err(_e) = file {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path);
        if let Ok(mut file) = file {
            let mut buf = Cursor::new(Vec::with_capacity(20480)); // 20kb buffer
            let url = instruction_cache_url(day);
            session.download(&url, &mut buf);
            buf.seek(SeekFrom::Start(0)).expect("Shouldn't fail");
            let doc = Document::from_read(buf).unwrap();
            for main in doc.find(Name("body").descendant(Name("main"))) {
                node_to_markdown(main, &mut file)?;
            }
            file.flush()?
        }
    }
    Ok(())
}

fn node_to_markdown<W: Write>(parent: Node, buf: &mut W) -> Result<(), std::io::Error> {
    for node in parent.children() {
        if let Some(name) = node.name() {
            match name {
                "article" => node_to_markdown(node, buf)?,
                "h2" => write!(buf, "## {}\n", node.text())?,
                "p" => write!(buf, "{}\n\n", node.text())?,
                "pre" => write!(buf, "\t{}\n", node.text())?,
                "ul" => {
                    write!(buf, "\n")?;
                    node_to_markdown(node, buf)?
                }
                "li" => write!(buf, "  * {}\n", node.text())?,
                _ => write!(buf, "\n<{}>\n", node.text())?,
            }
        }
    }
    Ok(())
}

/// Wrap all input & instructions requests
#[derive(Debug)]
pub struct Session {
    headers: HeaderMap,
    client: Client,
}

impl Session {
    pub fn new(token: &str) -> Session {
        let headers = Session::header(&token);
        let client = reqwest::Client::new();
        Session { headers, client }
    }
    pub fn from_file(filename: &str) -> Result<Session, std::io::Error> {
        let session_file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(filename)?;
    
        let mut session_reader = BufReader::new(session_file);
        let mut token = String::new();
        session_reader
            .read_line(&mut token)?;
        token = token.trim_end().to_string();
        Ok(Session::new(&token))
    }
    fn get(&self, url: &str) -> reqwest::Response {
        let request = self.client.get(&*url).headers(self.headers.clone()).send();
        let request = match request {
            Ok(c) => c,
            Err(e) => panic!("err:{}", e),
        };
        request
    }
    fn header(token: &str) -> HeaderMap {
        let mut session_raw = "session=".to_string();
        session_raw.push_str(&token);
        let mut headers = HeaderMap::new();
        let name = HeaderName::from_lowercase(b"cookie").expect("a would-be-const-function failed?");
        let value = match HeaderValue::from_str(&session_raw) {
            Ok(c) => c,
            Err(e) => panic!("error with your session token, {}", e),
        };
        headers.insert(name, value);
        headers
    }
    pub fn download_file(&self, url: &str, filename: &str) -> Result<File, std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;
        self.download(url, &mut file);
        Ok(file)
    }
    pub fn download<W: Write>(&self, url: &str, buffer: &mut W) {
        let mut request = self.get(&url);
        request.copy_to(buffer).unwrap();
        buffer.flush().unwrap();
    }
}
