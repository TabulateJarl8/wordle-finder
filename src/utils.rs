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

use regex::Regex;

pub fn find_matching_words(match_pattern: &Regex, exclude_regex: &Regex, include_list_string: &String) -> Vec<String> {
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

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn contains_all(word: &str, characters: &str) -> bool {
    // function to check if word contains all of characters in `characters`
    for char in characters.chars() {
        if ! word.contains(char) {
            return false;
        }
    }
    return true;
}
