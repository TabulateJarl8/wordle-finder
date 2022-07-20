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

use clap::{command, arg};
use regex::Regex;
use native_dialog::{MessageDialog, MessageType};

mod gui;
mod utils;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // clap setup
    let matches = command!()
        .arg(
            arg!(
                -p --pattern <PATTERN> "Regex pattern to search for. Use a `.` for unknown character. E.g.: `app.e`"
            )
            .required(false)
        )
        .arg(
            arg!(
                -e --exclude <EXCLUDE> "String of letters to exclude. E.g.: `ref`"
            )
            .required(false)
        )
        .arg(
            arg!(
                -i --include <INCLUDE> "String of letters to include. E.g.: `ref`"
            )
            .required(false)
        )
        .arg(
            arg!(
                -g --gui "Launch the WebView GUI"
            )
            .required(false)
        )
        
        .get_matches();

    // run gui if no args are specified (detecting a tty doesn't seem to work on windows)
    // this should launch the GUI if run from a file explorer
    if matches.is_present("gui") || std::env::args().len() == 1 {
        match gui::run_gui() {
            Err(error) => {
                let result = MessageDialog::new()
                    .set_title("Error")
                    .set_text(&format!("{:?}", &error))
                    .set_type(MessageType::Error)
                    .show_alert();

                match result {
                    Err(second_error) => {
                        eprintln!("Error when launching MessageDialog: {}", second_error);
                        panic!("Error when running GUI: {:?}", error);
                    },
                    _ => ()
                }

            },
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
