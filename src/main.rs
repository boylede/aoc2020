use aoc2020::Day;
use aoc2020::Session;

use std::process;

const USAGE: &'static str = "Dan Boyle's Advent of Code 2020 entries.
\t-d, --day (default 1) which day's code to run.
\t-a, --all run all day modules currently present.
\t-i, --input (string) optional input file.
\t-s, --session (string) optional session string.
\t-o, --offline stay offline, do a dry run";

#[derive(Debug, Clone)]
pub struct Config {
    pub day: i32,
    pub all: bool,
    pub offline: bool,
    pub session: String,
    pub input: String,
}

impl Config {
    pub fn new(args: lapp::Args) -> Result<Config, &'static str> {
        let all = args.get_bool("all");
        let offline = args.get_bool("offline");
        let day = args.get_integer("day");

        let session = match args.flag_present("session") {
            true => args.get_string("session"),
            false => "".to_string(),
        };

        let input = match args.flag_present("input") {
            true => args.get_string("input"),
            false => "".to_string(),
        };

        Ok(Config {
            all,
            offline,
            day,
            session,
            input,
        })
    }
}

fn main() {
    /* 	Parse Arguments */
    let mut args = lapp::Args::new(USAGE);
    match args.parse_result() {
        Ok(()) => (),
        Err(error) => {
            println!("Error parsing arguments: {}, try --help.", error);
            return;
        }
    }
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(0);
    });

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
            assert!(index == day.index as usize);
            run_day(day, &config);
        } else {
            println!("Invalid day selection: {}", config.day);
        }
    }
}

fn run_day(day: &Day, config: &Config) {
    println!("Running day: {}", &day);
    if !config.offline {
        let session = if config.session.len() > 0 {
            Session::new(&config.session)
        } else {
            Session::from_file("session.txt")
        };
        match session {
            Ok(session) => day.cache_input_and_run(&session),
            Err(_) => println!(
                "Please create a session.txt file or provide --session on the command line."
            ),
        }
    } else {
        day.run_with_cached_input();
    }
}
