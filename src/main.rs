//#![feature(collections)]

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

/// ```
/// let d = complement_dna("CCGTTAG")
/// assert!(d = "GGCAATC")
/// ```
fn complement_dna(dna: String) -> String {
    let mut comp = String::new();
    for b in dna.chars() {
        let bc = match b {
            'A' => 'T',
            'a' => 'T',
            'T' => 'A',
            't' => 'A',
            'C' => 'G',
            'c' => 'G',
            'G' => 'C',
            'g' => 'G',
            _ => 'N'
        };
        comp.push(bc);
    }
    comp
}

fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

fn reverse_complement(dna: String) -> String {
    reverse(complement_dna(dna))
}

static USAGE: &'static str = "
Usage: rc (<seq>... | -f <file>)

Options:
    -f  File containing DNA sequences
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: String,
    arg_seq: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.arg_seq.len() > 0 {
        for s in args.arg_seq {
            let c = reverse_complement(s);
            println!{"{}", c};
        }
    } else {
        let path = Path::new(&args.arg_file);
        let file = match File::open(&path) {
            Err(why) => panic!("Unable to open {}: {}", path.display(),
                                                        Error::description(&why)),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let c = reverse_complement(line.unwrap());
            println!{"{}", c};
        }
    }
}
