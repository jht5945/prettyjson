extern crate argparse;
extern crate term;
extern crate json;
extern crate rust_util;

mod opt;

use std::{
    fs::{self, File},
    io::{
        self,
        prelude::*,
    }
};
use rust_util::{
    XResult,
    util_msg::*,
};
use opt::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"prettyjson {} - {}
Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
}

fn read_to_string(read: &mut dyn Read) -> XResult<String> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    Ok(buffer)
}


fn main() {
    let mut options = Options::new();
    options.parse_args();
    
    if options.verbose {
        print_message(MessageType::DEBUG, &format!("prettyjson version: {}, git hash: {}", VERSION, GIT_HASH));
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
