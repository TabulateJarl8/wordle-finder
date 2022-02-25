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

use serde::Deserialize;
use regex::Regex;
use wry::{
    application::window::{Window, WindowId},
    webview::WebView,
};
use std::{
    collections::HashMap,
    cell::RefCell,
};
use crate::{
    utils::find_matching_words,
    VERSION,
};

thread_local! {
    static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
}

pub fn run_gui() -> wry::Result<()> {
    use wry::{
        application::{
          event::{Event, WindowEvent},
          event_loop::{ControlFlow, EventLoop},
          window::{WindowBuilder, Icon},
          dpi::LogicalSize,
        },
        webview::WebViewBuilder,
    };

    use image::ImageFormat;

    let html_content = include_str!("../ui/index.html");

    // load icon
    let bytes: Vec<u8> = include_bytes!("../img/wordle_finder_icon.png").to_vec();
    let imagebuffer = image::load_from_memory_with_format(&bytes, ImageFormat::Png).unwrap().into_rgba8();
    let (icon_width, icon_height) = imagebuffer.dimensions();
    let icon_rgba = imagebuffer.into_raw();

    // build window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("Wordle Finder v{}", VERSION))
        .with_inner_size(LogicalSize::new(990.0, 720.0))
        .with_window_icon(Some(Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap()))
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

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Cmd {
    PopulateWordList { pattern: String, include: String, exclude: String }
}
