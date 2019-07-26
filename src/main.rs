extern crate argparse;
extern crate term;
extern crate json;
extern crate rust_util;

use std::{
    fs::File,
    io::{
        self,
        prelude::*,
    }
};
use argparse::{ArgumentParser, StoreTrue, Store};
use rust_util::*;

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

struct Options {
    version: bool,
    tab_width: u16,
    file: String,
}

fn main() {
    let mut options = Options {
        version: false,
        tab_width: 4u16,
        file: String::new(),
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("prettyjson - command line JSON pretty tool.");
        ap.refer(&mut options.tab_width).add_option(&["-w", "--tab-width"], Store, "Tab width, default 4");
        ap.refer(&mut options.version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.refer(&mut options.file).add_argument("FILE", Store, "FILE");
        ap.parse_args_or_exit();
    }
    
    if options.version {
        print_version();
        return;
    }

    if options.tab_width > 100 {
        print_message(MessageType::ERROR, &format!("Tab width is invalid: {}", options.tab_width));
        return;
    }

    let read: XResult<String> = match options.file.len() {
        0 => read_to_string(&mut io::stdin()),
        _ => match File::open(&options.file) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Open file: {}, failed: {}", &options.file, err));
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

    println!("{}", json::stringify_pretty(json_object, options.tab_width));
}
