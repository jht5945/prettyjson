extern crate argparse;
extern crate term;
extern crate json;
#[macro_use]
extern crate rust_util;

mod opt;

use std::{ io, fs, fs::File };
use rust_util::{ XResult, util_io::* };
use opt::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"{} {} - {}
Copyright (C) 2019-2020 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, NAME, VERSION, &GIT_HASH[0..7]);
}


fn main() {
    let options = Options::parse_args_static();
    
    if options.verbose {
        debugging!("{} version: {}, git hash: {}", NAME, VERSION, GIT_HASH);
    }
    
    if options.version {
        print_version();
        return;
    }

    if options.tab_width > 100 {
        failure!("Tab width is invalid: {}", options.tab_width);
        return;
    }

    let read: XResult<String> = match options.file.len() {
        0 => read_to_string(&mut io::stdin()),
        _ => match File::open(&options.file) {
            Ok(mut f) => read_to_string(&mut f),
            Err(err) => {
                failure!("Open file: {}, failed: {}", &options.file, err);
                return;
            },
        },
    };

    let json_object = match read {
        Ok(content) => match json::parse(&content) {
            Ok(json_object) => json_object,
            Err(err) => {
                failure!("Parse JSON failed: {}", err);
                return;
            },
        },
        Err(err) => {
            failure!("Read JSON failed: {}", err);
            return;
        },
    };

    let pretty_json = json::stringify_pretty(json_object, options.tab_width);
    println!("{}", pretty_json);

    if !options.file.is_empty() && options.replace_file {
        match fs::write(&options.file, pretty_json) {
            Err(err) => failure!("Write JSON file failed: {}", err),
            Ok(_) =>  success!("Write JSON file success."),
        }
    }
}
