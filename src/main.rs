use clap::{Arg, App};
use regex::Regex;
use web_view::*;

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

    web_view::builder()
        .title("Wordle Finder")
        .content(Content::Html(html_content))
        .size(990, 720)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    let matches = App::new("Wordle Finder")
                    .version("1.2.0")
                    .author("Connor Sample (TabulateJarl8)")
                    .about("Helper tool to narrow down choices for wordle word")
                    .arg(Arg::new("pattern")
                        .short('p')
                        .long("pattern")
                        .help("Regex pattern to search for. Use a `.` for unknown character. E.g.: `app.e`")
                        .takes_value(true)
                        .required(true))
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
                    .get_matches();

    // get regex pattern
    let pattern: String = matches.value_of("pattern").unwrap().to_lowercase();
    let match_pattern: Regex = Regex::new(&pattern).unwrap();

    // parse exclude list; remove all whitespace and covert to lowercase to prepare for split
    let exclude_list_string: String = remove_whitespace(matches.value_of("exclude").unwrap_or("/")).to_lowercase();
    let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude_list_string)).unwrap();

    // parse include list; remove all whitespace and covert to lowercase to prepare for split
    let include_list_string: String = remove_whitespace(matches.value_of("include").unwrap_or("")).to_lowercase();

    for word in words_list {
        if exclude_regex.is_match(word) {
            if contains_all(&word, &include_list_string) {
                if match_pattern.is_match(word) {
                    println!("{}", word);
                }
            }
        }
    }

}
