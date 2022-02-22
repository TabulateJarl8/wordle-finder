// Wordle Finder is a cross-platform CLI/GUI utility to assist in finding the word for the game "Wordle" and other similar knock-offs
// Copyright (C) 2022 Connor Sample
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{Arg, Command};
use regex::Regex;

mod gui;
mod utils;

// TODO: Bundle WebView2Loader.dll for windows users

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // clap setup
    let matches = Command::new("Wordle Finder")
                    .version(format!("v{}", VERSION).as_str())
                    .author("Connor Sample (TabulateJarl8)")
                    .about("A cross-platform CLI/GUI utility to assist in finding the word for the game \"Wordle\" and other similar knock-offs. Licensed under GPLv3.")
                    .arg(Arg::new("pattern")
                        .short('p')
                        .long("pattern")
                        .help("Regex pattern to search for. Use a `.` for unknown character. E.g.: `app.e`")
                        .takes_value(true)
                        .required(false))
                    .arg(Arg::new("exclude")
                        .short('e')
                        .long("exclude")
                        .help("String of letters to exclude. E.g.: `ref`")
                        .takes_value(true)
                        .required(false))
                    .arg(Arg::new("include")
                        .short('i')
                        .long("include")
                        .help("String of letters to include. E.g.: `ref`")
                        .takes_value(true)
                        .required(false))
                    .arg(Arg::new("gui")
                        .short('g')
                        .long("gui")
                        .help("Launch the WebView GUI")
                        .takes_value(false)
                        .required(false))
                    .get_matches();

    // run gui if specified or if not a tty (not running from a terminal)
    if matches.is_present("gui") || !atty::is(atty::Stream::Stdout) {
        match gui::run_gui() {
            Err(error) => panic!("Error when running GUI: {:?}", error),
            _ => (),
        };
        std::process::exit(0);
    }

    // get regex pattern
    let pattern: String = matches.value_of("pattern").unwrap_or(".....").to_lowercase();
    let match_pattern: Regex = Regex::new(&pattern).unwrap();

    // parse exclude list; remove all whitespace and covert to lowercase to prepare for split
    let exclude_list_string: String = utils::remove_whitespace(matches.value_of("exclude").unwrap_or("/")).to_lowercase();
    let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude_list_string)).unwrap();

    // parse include list; remove all whitespace and covert to lowercase to prepare for split
    let include_list_string: String = utils::remove_whitespace(matches.value_of("include").unwrap_or("")).to_lowercase();

    let matching_words = utils::find_matching_words(&match_pattern, &exclude_regex, &include_list_string);
    for word in matching_words {
        println!("{}", word);
    }
}
