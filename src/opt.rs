
use argparse::{ArgumentParser, StoreTrue, Store};

pub struct Options {
    pub version: bool,
    pub verbose: bool,
    pub replace_file: bool,
    pub tab_width: u16,
    pub file: String,
}

impl Options {
    pub fn new() -> Options {
        Options {
            version: false,
            verbose: false,
            replace_file: false,
            tab_width: 4u16,
            file: String::new(),
        }
    }

    pub fn parse_args(&mut self) {
        let mut ap = ArgumentParser::new();
        ap.set_description("prettyjson - command line JSON pretty tool.");
        ap.refer(&mut self.tab_width).add_option(&["-w", "--tab-width"], Store, "Tab width, default 4");
        ap.refer(&mut self.version).add_option(&["-V", "--version"], StoreTrue, "Print version");
        ap.refer(&mut self.verbose).add_option(&["-v", "--verbose"], StoreTrue, "Verbose");
        ap.refer(&mut self.replace_file).add_option(&["-R", "--replace-file"], StoreTrue, "Replace file");
        ap.refer(&mut self.file).add_argument("FILE", Store, "FILE");
        ap.parse_args_or_exit();
    }
}