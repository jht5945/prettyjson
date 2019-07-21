extern crate argparse;
extern crate term;
extern crate json;

mod util;

use std::{
    fs::File,
    io::{
        self,
        prelude::*,
    }
};
use argparse::{ArgumentParser, StoreTrue, Store};
use util::*;

const VERSION: &str = "0.1";

fn print_version() {
    print!(r#"prettyjson {}

Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>
Written by Hatter Jiang
"#, VERSION);
}

fn read_to_string(read: &mut Read) -> XResult<String> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let mut version = false;
    let mut tab_width = 4u16;
    let mut file = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("prettyjson - command line JSON pretty tool.");
        ap.refer(&mut tab_width).add_option(&["-w", "--tab-width"], Store, "Tab width, default 4");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.refer(&mut file).add_argument("FILE", Store, "FILE");
        ap.parse_args_or_exit();
    }
    
    if version {
        print_version();
        return;
    }

    if tab_width > 100 {
        print_message(MessageType::ERROR, &format!("Tab width is invalid: {}", tab_width));
        return;
    }

    let read: XResult<String> = match file.len() {
        0 => read_to_string(&mut io::stdin()),
        _ => match File::open(&file) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Open file: {}, failed: {}", &file, err));
                return;
            },
            Ok(mut f) => read_to_string(&mut f),
        },
    };

    let json_object = match read {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Read JSON failed: {}", err));
            return;
        },
        Ok(content) => match json::parse(&content) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Parse JSON pailed: {}", err));
                return;
            },
            Ok(json_object) => json_object,
        },
    };

    println!("{}", json::stringify_pretty(json_object, tab_width));
}
