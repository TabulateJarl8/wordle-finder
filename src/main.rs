use clap::{Arg, Command};
use regex::Regex;
use serde::Deserialize;
use std::{
    cell::RefCell,
    collections::HashMap
};
use wry::{
    application::window::{Window, WindowId},
    webview::WebView,
};

// TODO: Bundle WebView2Loader.dll for windows users

thread_local! {
    static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
}

static VERSION: &str = "2.2.0";

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
    // clap setup
    let matches = Command::new("Wordle Finder")
                    .version(VERSION)
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

    // run gui if specified or if not a tty (not running from a terminal)
    if matches.is_present("gui") || !atty::is(atty::Stream::Stdout) {
        match run_gui() {
            Err(error) => panic!("Error when running GUI: {:?}", error),
            _ => (),
        };
        std::process::exit(0);
    }

    // get regex pattern
    let pattern: String = matches.value_of("pattern").unwrap_or(".....").to_lowercase();
    let match_pattern: Regex = Regex::new(&pattern).unwrap();

    // parse exclude list; remove all whitespace and covert to lowercase to prepare for split
    let exclude_list_string: String = remove_whitespace(matches.value_of("exclude").unwrap_or("/")).to_lowercase();
    let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude_list_string)).unwrap();

    // parse include list; remove all whitespace and covert to lowercase to prepare for split
    let include_list_string: String = remove_whitespace(matches.value_of("include").unwrap_or("")).to_lowercase();

    let matching_words = find_matching_words(&match_pattern, &exclude_regex, &include_list_string);
    for word in matching_words {
        println!("{}", word);
    }
}

fn run_gui() -> wry::Result<()> {
    use wry::{
        application::{
          event::{Event, WindowEvent},
          event_loop::{ControlFlow, EventLoop},
          window::WindowBuilder,
          dpi::LogicalSize,
        },
        webview::WebViewBuilder,
    };

    // build window
    let html_content = include_str!("../ui/index.html");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("Wordle Finder v{}", VERSION))
        .with_inner_size(LogicalSize::new(990.0, 720.0))
        .build(&event_loop)?;
    let window_id = window.id();

    // build webview
    let webview = WebViewBuilder::new(window)?
        .with_html(html_content)?
        .with_ipc_handler(ipc_handler)
        .build()?;

    // add webview and window ID to WEBVIEWS refcell hashmap
    let mut webview_hashmap: HashMap<WindowId, WebView> = HashMap::new();
    webview_hashmap.insert(window_id, webview);

    WEBVIEWS.with(|webviews| {
        *webviews.borrow_mut() = webview_hashmap;
    });


    // run event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn ipc_handler(window: &Window, message: String) {
    use Cmd::*;

    // match incoming IPC message
    match serde_json::from_str(&message).unwrap() {
        PopulateWordList { pattern, include, exclude } => {
            // parse JSON payload
            let match_pattern: Regex = Regex::new(&pattern.to_lowercase()).unwrap();
            let exclude_regex: Regex = Regex::new(&format!("^[^{}]+$", exclude.to_lowercase())).unwrap();
            let include_string = include.to_lowercase();
            let matching_words = find_matching_words(&match_pattern, &exclude_regex, &include_string);

            // send response back to client
            WEBVIEWS.with(|webviews| {
                let webviews = webviews.borrow();

                if let Some(wv) = webviews.get(&window.id()) {
                    let result = wv.evaluate_script(&format!("window.populateWordList({:?})", matching_words));
                    match result {
                        Err(error) => println!("Error when sending response to client (populateWordList): {:?}", error),
                        _ => (),
                    }
                }
            })
        }
    }
}

fn find_matching_words(match_pattern: &Regex, exclude_regex: &Regex, include_list_string: &String) -> Vec<String> {
    let words = include_str!("words.txt");
    let words_list: Vec<&str> = words.split('\n').collect();
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
