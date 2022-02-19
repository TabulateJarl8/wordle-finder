use clap::{Arg, App};
use regex::Regex;
use web_view::*;
use serde::Deserialize;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn contains_all(word: &str, characters: &str) -> bool {
    // function to check if word contains all of characters in `characters`
    for char in characters.chars() {
        if ! word.contains(char) {
            return false;
        }
    }
    return true;
}

fn main() {
    let words = include_str!("words.txt");
    let html_content = include_str!("../ui/index.html");
    let words_list: Vec<&str> = words.split('\n').collect();

    let matches = App::new("Wordle Finder")
                    .version("1.2.0")
                    .author("Connor Sample (TabulateJarl8)")
                    .about("Helper tool to narrow down choices for wordle word")
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

    if matches.is_present("gui") {
        web_view::builder()
            .title("Wordle Finder")
            .content(Content::Html(html_content))
            .size(990, 720)
            .resizable(false)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                use Cmd::*;

                match serde_json::from_str(arg).unwrap() {
                    PopulateWordList { pattern, include, exclude } => {
                        let match_pattern: Regex = Regex::new(&pattern.to_lowercase()).unwrap();
                        let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude.to_lowercase())).unwrap();
                        let include_string = include.to_lowercase();
                        let matching_words = find_matching_words(&match_pattern, &exclude_regex, &include_string, &words_list);
                        webview.eval(&format!("populateWordList({:?})", matching_words))
                    },
                }


            })
            .run()
            .unwrap();

        std::process::exit(0x0100);
    }

    // get regex pattern
    let pattern: String = matches.value_of("pattern").unwrap_or(".....").to_lowercase();
    let match_pattern: Regex = Regex::new(&pattern).unwrap();

    // parse exclude list; remove all whitespace and covert to lowercase to prepare for split
    let exclude_list_string: String = remove_whitespace(matches.value_of("exclude").unwrap_or("/")).to_lowercase();
    let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude_list_string)).unwrap();

    // parse include list; remove all whitespace and covert to lowercase to prepare for split
    let include_list_string: String = remove_whitespace(matches.value_of("include").unwrap_or("")).to_lowercase();

    let matching_words = find_matching_words(&match_pattern, &exclude_regex, &include_list_string, &words_list);
    for word in matching_words {
        println!("{}", word);
    }
}

fn find_matching_words(match_pattern: &Regex, exclude_regex: &Regex, include_list_string: &String, words_list: &Vec<&str>) -> Vec<String> {
    let mut matching_words: Vec<String> = Vec::new();

    for word in words_list {
        if exclude_regex.is_match(word) {
            if contains_all(&word, &include_list_string) {
                if match_pattern.is_match(word) {
                    matching_words.push(String::from(word.clone()));
                }
            }
        }
    }
    matching_words
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    PopulateWordList { pattern: String, include: String, exclude: String }
}
