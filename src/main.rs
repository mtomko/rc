#![feature(collections)]
#![feature(env)]

use std::env;

/// ```
/// let d = complement_dna("CCGTTAG")
/// assert!(d = "GGCAATC")
/// ```
fn complement_dna(dna: &str) -> Box<String> {
    let mut comp = Box::new(String::new());
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

fn reverse(s: Box<String>) -> Box<String> {
    Box::new(s.chars().rev().collect::<String>())
}

fn reverse_complement(dna: &str) -> Box<String> {
    reverse(complement_dna(dna))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for s in args.tail() {
        let c = reverse_complement(&s);
        println!{"{}", c};
    }

}
