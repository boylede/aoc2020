/// All code related to loading and running each day
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};

use serde::{Deserialize, Serialize};

const YEAR: i32 = 2020;
const AOC_URL: &str = "https://adventofcode.com/";

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

macro_rules! day_element {
    ($module:ident, $number:expr) => {
        Day {
            index: $number,
            part1: $module::part1,
            part2: $module::part2,
        }
    };
}

pub const DAYS: &[Day] = &[
    day_element!(day1, 1),
    day_element!(day2, 2),
    day_element!(day3, 3),
    day_element!(day4, 4),
    day_element!(day5, 5),
    day_element!(day6, 6),
];

pub enum RunError {
    SessionFailed(SessionError),
    CacheInError,
    CacheOutError,
    InputError,
    DayError(String),
}

pub type RunResult = Result<(String, String), RunError>;

#[derive(Debug)]
pub enum PartError {
    Unimplemented,
    Failed(String),
}

pub type PartResult = Result<String, PartError>;

pub type PartFunction = fn(&Vec<String>) -> PartResult;

/// Wrap the day's runner function so we can store all loaded days in a vec
pub struct Day {
    part1: PartFunction,
    part2: PartFunction,
    pub index: i32,
}

impl Day {
    pub fn run(self: &Self, input: File) -> RunResult {
        println!("Loading day {} input.", self.index);
        let mut lines = vec![];
        {
            let mut lines_iterator = BufReader::new(&input).lines();
            while let Some(Ok(line)) = lines_iterator.next() {
                lines.push(line);
            }
        }
        let a_time = time::precise_time_ns();
        let part1 = (self.part1)(&lines);
        println!(
            "Part 1 result: {}",
            part1.as_ref().map(|s| s.as_str()).unwrap_or("failed!")
        );
        let b_time = time::precise_time_ns();
        let part2 = (self.part2)(&lines);
        println!(
            "Part 2 result: {}",
            part2.as_ref().map(|s| s.as_str()).unwrap_or("failed!")
        );
        let c_time = time::precise_time_ns();
        let p1_time = b_time - a_time;
        let ns_in_ms = 1_000_000;
        print!("Day {} Part 1 took: ", self.index);
        if p1_time > ns_in_ms {
            println!("{} ms", p1_time / ns_in_ms);
        } else {
            println!("{} ns", p1_time);
        }
        let p2_time = c_time - b_time;
        print!("Day {} Part 2 took: ", self.index);
        if p2_time > ns_in_ms {
            println!("{} ms", p2_time / ns_in_ms);
        } else {
            println!("{} ns", p2_time);
        }

        use RunError::DayError;
        match (part1, part2) {
            (Ok(result1), Ok(result2)) => Ok((result1, result2)),
            (Err(err1), Ok(result2)) => Err(DayError(format!(
                "part 1 error: {:?}, part 2 success: {}",
                err1, result2
            ))),
            (Ok(result1), Err(err2)) => Err(DayError(format!(
                "part 1 success: {}, part 2 error: {:?}",
                result1, err2
            ))),
            (Err(err1), Err(err2)) => Err(DayError(format!(
                "part 1 error: {:?}, part 2 error: {:?}",
                err1, err2
            ))),
        }
    }
    pub fn clear_cache(&self) {
        let file_path = input_cache_path(self.index);
        match fs::remove_file(file_path) {
            Ok(_) => (),
            Err(e) => println!("error deleting file: {}", e),
        };
        let ins_file_path = instruction_cache_path(self.index);
        match fs::remove_file(ins_file_path) {
            Ok(_) => (),
            Err(e) => println!("error deleting file: {}", e),
        };
    }
    pub fn cache_input_and_run(&self, session: &Session) -> RunResult {
        let input =
            cache_files(self.index, &session).map_err(|err| RunError::SessionFailed(err))?;
        self.run(input)
    }
    pub fn run_with_cached_input(&self) -> RunResult {
        let file_path = input_cache_path(self.index);
        let input = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&file_path)
            .map_err(|_| RunError::CacheInError)?;
        self.run(input)
    }
    pub fn run_with_test_input(&self, input_filename: &str) -> RunResult {
        let input = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(input_filename)
            .map_err(|_| RunError::InputError)?;
        self.run(input)
    }
    pub fn cache_result(&self, result: (String, String)) -> Result<(), RunError> {
        let file_path = result_cache_path(self.index);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path)
            .map_err(|_| RunError::CacheOutError)?;
        let results = Results(result.0, result.1);
        ron::ser::to_writer(&mut file, &results).map_err(|_| RunError::CacheOutError)?;
        println!("Cached results.");
        Ok(())
    }
    pub fn validate_result(&self, result: (String, String)) -> Result<(), RunError> {
        let file_path = result_cache_path(self.index);
        let mut file = fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open(&file_path)
            .map_err(|_| RunError::CacheOutError)?;
        let cache: Results =
            ron::de::from_reader(&mut file).map_err(|_| RunError::CacheOutError)?;
        if cache.0 == result.0 && cache.1 == result.1 {
            println!("Results Validated!");
        } else {
            println!("Results do NOT match cache.");
        }
        Ok(())
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
pub fn result_cache_path(day: i32) -> String {
    format!("results/day{:02}.txt", day)
}

#[derive(Serialize, Deserialize)]
pub struct Results(String, String);

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
                "h2" => {
                    let mut text = node.text();
                    text = text
                        .trim_end_matches("---")
                        .trim_start_matches("---")
                        .trim_end()
                        .trim_start()
                        .to_string();
                    write!(buf, "## {}\n", text)?
                }
                "p" => {
                    let text = node.text();
                    if !text.starts_with("You can also [Shareon") {
                        write!(buf, "{}\n", node.text())?
                    }
                }
                "pre" => {
                    write!(buf, "~~~")?;
                    for line in node.text().split('\n') {
                        write!(buf, "\n{}", line)?;
                    }
                    write!(buf, "~~~\n")?;
                }
                "ul" => {
                    write!(buf, "\n")?;
                    node_to_markdown(node, buf)?
                }
                "li" => write!(buf, "  * {}\n", node.text())?,
                "script" => (),
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
