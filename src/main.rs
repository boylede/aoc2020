use aoc2020::Day;
use aoc2020::Session;

use clap::Clap;

/// Advent of Code 2020 entries
#[clap(version = "0.1.0", author = "Daniel Boyle")]
#[derive(Debug, Clone, Clap)]
pub struct Config {
    /// which day to run
    #[clap(short = 'd', long = "day", default_value = "1")]
    pub day: i32,
    /// run all days, ignores -day if set
    #[clap(short = 'a', long = "all")]
    pub all: bool,
    /// dry-run the program offline
    #[clap(short = 'o', long = "offline")]
    pub offline: bool,
    /// provide a session token on the command line or in a session.txt file
    #[clap(short = 's', long = "session")]
    pub session: Option<String>,
    /// provide alternate input for testing
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
}

fn main() {
    /* 	Parse Arguments */
    let config = Config::parse();

    /* 	Main Logic */
    let days = aoc2020::DAYS;
    if config.all {
        for day in days {
            run_day(day, &config);
        }
    } else {
        let index = (config.day - 1) as usize;
        if index < days.len() {
            let day = &days[index];
            assert!(index + 1 == day.index as usize);
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
        match session {
            Ok(session) => day.cache_input_and_run(&session),
            Err(_) => println!(
                "Please create a session.txt file or provide --session on the command line."
            ),
        }
    } else if config.input == None {
        day.run_with_cached_input();
    } else {
        let input_filename = config.input.as_ref().expect("unreachable");
            day.run_with_test_input(&input_filename);
    }
}

