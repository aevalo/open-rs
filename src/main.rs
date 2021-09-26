extern crate argparse;

use argparse::{ArgumentParser, Print, Store, StoreTrue};
use std::env;

fn print_env_var(var_name: &str) {
    match env::var(var_name) {
        Err(e) => println!(
            "Caught error reading environment variable {}: {}",
            var_name, e
        ),
        Ok(var_value) => println!("${} is {}", var_name, var_value),
    }
}

struct Options {
    verbose: bool,
    file: String,
}

fn main() {
    let mut options = Options {
        verbose: false,
        file: String::new(),
    };

    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Open files from command line based on file type.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.add_option(
            &["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );
        ap.refer(&mut options.file)
            .add_argument("file", Store, "File to open")
            .required();
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }

    if options.verbose {
        print_env_var("XDG_DATA_HOME");
        print_env_var("XDG_CONFIG_HOME");
        print_env_var("XDG_DATA_DIRS");
        print_env_var("XDG_CONFIG_DIRS");
        print_env_var("XDG_CACHE_HOME");
        print_env_var("HOME");
    }

    println!("Opening file {}...", options.file);
}
