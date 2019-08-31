extern crate argparse;
extern crate term;
extern crate json;
extern crate rust_util;

mod opt;

use std::{
    fs::{self, File},
    io::{self}
};
use rust_util::{
    XResult,
    util_msg::*,
    util_io::*,
};
use opt::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"{} {} - {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, NAME, VERSION, &GIT_HASH[0..7]);
}


fn main() {
    let mut options = Options::new();
    options.parse_args();
    
    if options.verbose {
        print_message(MessageType::DEBUG, &format!("{} version: {}, git hash: {}", NAME, VERSION, GIT_HASH));
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
                print_message(MessageType::ERROR, &format!("Parse JSON failed: {}", err));
                return;
            },
            Ok(json_object) => json_object,
        },
    };

    let pretty_json = json::stringify_pretty(json_object, options.tab_width);
    println!("{}", pretty_json);

    if options.file.len() > 0 && options.replace_file {
        match fs::write(&options.file, pretty_json) {
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Write JSON file failed: {}", err));
            },
            Ok(_) => {
                print_message(MessageType::OK, "Write JSON file success.");
            },
        }
    }
}
