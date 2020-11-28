use aoc2020::{Day, PartResult, RunError, RunResult, Session, SessionError, cache_result_for_day};
use clap::Clap;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

/// Advent of Code 2020 entries
#[clap(version = "0.1.0", author = "Daniel Boyle")]
#[derive(Debug, Clone, Clap)]
pub struct Config {
    /// Which day to run.
    #[clap(short = 'd', long = "day", default_value = "1")]
    pub day: i32,
    /// Run all days, ignores --day if set.
    #[clap(short = 'a', long = "all")]
    pub all: bool,
    /// Dry-run the program offline.
    #[clap(short = 'o', long = "offline")]
    pub offline: bool,
    /// Provide session token on the command line or in a session.txt file.
    #[clap(short = 's', long = "session")]
    pub session: Option<String>,
    /// Provide alternate input for testing.
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
    /// Cache result for regression testing.
    #[clap(long = "accept")]
    pub accept: bool,
}

fn main() {
    let config = Config::parse();
    let days = aoc2020::DAYS;
    if config.all {
        for day in days {
            run_day(day, &config);
        }
    } else {
        let index = (config.day - 1) as usize;
        if index < days.len() {
            let day = &days[index];
            run_day(day, &config);
        } else {
            println!("Invalid day selection: {}", config.day);
        }
    }
}

fn run_day(day: &Day, config: &Config) {
    println!("Running day: {}", &day);
    if !config.offline && config.input == None {
        let session = if let Some(session) = &config.session {
            Session::new(&session)
        } else {
            Session::from_file("session.txt")
        };
        if let Ok(session) = session {
            let output = day.cache_input_and_run(&session);
            match output {
                Ok(result) => {
                    if config.accept {
                        cache_result_for_day(day.index, result);
                    }
                }
                Err(e) => print_error(e),
            }
        } else {
            println!("Please create a session.txt file or provide --session on the command line.");
        }
    } else if config.input == None {
        let output = day.run_with_cached_input();
        match output {
            Ok(result) => {
                if config.accept {
                    cache_result_for_day(day.index, result);
                }
            }
            Err(e) => print_error(e),
        }
    } else {
        let input_filename = config.input.as_ref().expect("unreachable");
        let output = day.run_with_test_input(&input_filename);
        if let Err(e) = output {
            print_error(e);
        }
    }
}

fn print_error(err: RunError) {
    use RunError::*;
    use SessionError::*;
    match err {
        SessionFailed(TokenFormat) => println!("Session token was unreadable."),
        SessionFailed(IoError(desc)) => println!("{}", desc),
        SessionFailed(NetworkError) => println!("Network request failed."),
        SessionFailed(BufferError) => println!("An error occured while writing memory."),
        SessionFailed(DomError) => println!("Unable to parse DOM."),
        CacheError => println!("No cached input available."),
        InputError => println!("Couldn't open test input file."),
        DayError(reason) => println!("Errors with this Day: {}", reason),
    }
}

