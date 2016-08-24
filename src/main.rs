extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Copy standard input to each FILE, and also to standard output.

Usage: 
	rtee [options] [<file>...]

Options:
  -a --append               append to the given FILEs, do not overwrite.
  -V --version              output version information and exit.
  -h --help                 display this help and exit.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Vec<String>,
    flag_append: bool,
    flag_version: bool,
    flag_help: bool,
}


fn main() {
    // load version into its expected form
    let version = Some(String::from(VERSION));
    // unwrap command line arguments
    let args: Args = Docopt::new(USAGE)
                            // -> Result<Docopt, Error>
                             .and_then(|d| Ok(d.version(version)))
                            // -> Result<Docopt, Error>
                             .and_then(|d| d.decode())
                            // -> Result<Args, Error>
                             .unwrap_or_else(|e| e.exit());
                            // -> Args
    println!("{:?}", args);
}
