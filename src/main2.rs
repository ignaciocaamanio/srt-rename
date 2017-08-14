extern crate regex;
extern crate glob;

use regex::Regex;
use std::fs;
use glob::glob;

fn main() {

    fn foo() -> std::io::Result<()> {

        let re = Regex::new(r".*([sS][0-9][0-9][eE][0-9][0-9]).*").unwrap();

        for entry in glob("*.srt").expect("Failed to read glob") {
            let dir = entry?;
            if re.is_match(&dir.file_name().into_string().unwrap()) {
                println!("{:?}", dir.path());
            }
        }

        Ok(())
    }

    foo();
}
