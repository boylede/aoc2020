use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    blocking::Client,
};

use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};

const YEAR: i32 = 2020;
const AOC_URL: &str = "https://adventofcode.com/";

pub mod day1;

pub const DAYS: &[Day] = &[Day {
    index: 1,
    runner: day1::run,
}];

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
        match file {
            Ok(input) => self.run(input),
            Err(SessionError::TokenFormat) => {
                println!("Session token was unreadable.");
            }
            Err(SessionError::IoError(desc)) => {
                println!("{}", desc);
            }
            Err(SessionError::NetworkError) => {
                println!("Network request failed");
            }
            Err(SessionError::BufferError) => {
                println!("An error occured while writing memory.");
            }
            Err(SessionError::DomError) => {
                println!("Unable to parse DOM");
            }
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
        if let Ok(input) = file {
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

pub fn cache_files(day: i32, session: &Session) -> Result<File, SessionError> {
    cache_instructions_for_day(day, &session)?;
    cache_input_for_day(day, &session)
}

pub fn input_cache_path(day: i32) -> String {
    format!("input/day{}.txt", day)
}
pub fn input_url(day: i32) -> String {
    format!("{}{}/day/{}/input", AOC_URL, YEAR, day)
}
pub fn instruction_cache_path(day: i32) -> String {
    format!("instructions/day{}.md", day)
}
pub fn instruction_cache_url(day: i32) -> String {
    format!("{}{}/day/{}", AOC_URL, YEAR, day)
}

pub fn cache_input_for_day(day: i32, session: &Session) -> Result<File, SessionError> {
    let file_path = input_cache_path(day);
    let file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_path);
    let url = input_url(day);
    match file {
        Ok(content) => Ok(content), // necessary to convert Result types
        Err(_) => {
            println!("Downloading inputs for day {}.", day);
            session.download_file(&url, &file_path)
        }
    }
}

pub fn cache_instructions_for_day(day: i32, session: &Session) -> Result<(), SessionError> {
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
            session.download(&url, &mut buf)?;
            buf.seek(SeekFrom::Start(0)).expect("infallible");
            let doc = Document::from_read(buf).map_err(|_| SessionError::DomError)?;
            for main in doc.find(Name("body").descendant(Name("main"))) {
                node_to_markdown(main, &mut file).map_err(|_| SessionError::DomError)?;
            }
            file.flush().map_err(|_| {
                SessionError::IoError(format!("Unable to close file: {}", file_path))
            })?;
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

pub enum SessionError {
    TokenFormat,
    IoError(String),
    NetworkError,
    BufferError,
    DomError,
}

impl std::convert::From<reqwest::header::InvalidHeaderValue> for SessionError {
    fn from(_: reqwest::header::InvalidHeaderValue) -> Self {
        SessionError::TokenFormat
    }
}

impl std::convert::From<reqwest::Error> for SessionError {
    fn from(_: reqwest::Error) -> Self {
        SessionError::NetworkError
    }
}

/// Wrap all input & instructions requests
#[derive(Debug)]
pub struct Session {
    headers: HeaderMap,
    client: Client,
}

impl Session {
    pub fn new(token: &str) -> Result<Session, SessionError> {
        let headers = Session::header(&token)?;
        let client = reqwest::blocking::Client::new();
        Ok(Session { headers, client })
    }
    pub fn from_file(filename: &str) -> Result<Session, SessionError> {
        let session_file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(filename)
            .map_err(|_| {
                SessionError::IoError(format!(
                    "unable to load session token from file {}",
                    filename
                ))
            })?;

        let mut session_reader = BufReader::new(session_file);
        let mut token = String::new();
        session_reader
            .read_line(&mut token)
            .map_err(|_| SessionError::IoError(format!("unable to read from file {}", filename)))?;
        token = token.trim_end().to_string();
        Ok(Session::new(&token)?)
    }
    fn header(token: &str) -> Result<HeaderMap, SessionError> {
        let mut session_raw = "session=".to_string();
        session_raw.push_str(&token);
        let mut headers = HeaderMap::new();
        let name = HeaderName::from_lowercase(b"cookie").expect("infallible");
        let value = HeaderValue::from_str(&session_raw)?;
        headers.insert(name, value);
        Ok(headers)
    }
    pub fn download_file(&self, url: &str, filename: &str) -> Result<File, SessionError> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .map_err(|_| {
                SessionError::IoError(format!("unable to open cache file file {}", filename))
            })?;
        self.download(url, &mut file)?;
        Ok(file)
    }
    pub fn download<W: Write>(&self, url: &str, buffer: &mut W) -> Result<(), SessionError> {
        let response = self.client.get(&*url).headers(self.headers.clone()).send();
        let mut content = response?;
        content
            .copy_to(buffer)
            .map_err(|_| SessionError::BufferError)?;
        buffer.flush().map_err(|_| SessionError::BufferError)?;
        Ok(())
    }
}
