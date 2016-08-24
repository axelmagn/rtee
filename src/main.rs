extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions, File};



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
                            


    // set up open opts
    let append = args.flag_append;
    let open_opts = { 
        let mut o = OpenOptions::new();
        o.read(false)
         .write(true)
         .create(true)
         .append(append)
         .truncate(!append);
        o
    };

    // Open files for writing
    let mut files: Vec<File> = args.arg_file.iter()
                                     .map(|p| open_opts.open(p).unwrap())
                                     .collect();

    // open stderr and stdout for writing
    // let stderr = io::stderr();
    let mut stdout = io::stdout();

    // consume stdin
    io::stdin().bytes()
        // handle errors by printing them to stderr
        .map(|b| b.unwrap())
        // write byte to each file
        // (ownership fail)
        .map(|b| {
            // write to each file
            // fail loudly for now
            files.iter_mut()
                .map(|f: &mut File| f.write(&[b]).unwrap())
                .last();
            b
        })
        // write to stdout
        .map(|b| stdout.write(&[b]).unwrap())
        .last();
}
