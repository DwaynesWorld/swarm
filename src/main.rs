extern crate num_cpus;

use globwalk::*;
use rayon::prelude::*;
use std::result::Result;
use std::time::{Instant, SystemTime};

const PATTERNS: &'static [&'static str] =
    &["*.{vw,rv,sl,dg,src,dd,pkg,mod,cls,cls,bpo,rpt,mnu,cal,con}"];

fn main() {
    println!("{} cores", num_cpus::get());
    println!("{} cpus", num_cpus::get_physical());

    let now = Instant::now();

    let base_dir = "/Users/KT/Documents/HeavyBid";
    let _ = build_index(base_dir);

    println!("{:?}", now.elapsed())
}

fn build_index(base_dir: &str) {
    // let walker: Vec<_> =
    GlobWalkerBuilder::from_patterns(base_dir, PATTERNS)
        .case_insensitive(true)
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .for_each(|e| index_file(e));
}

fn index_file(dir_entry: &DirEntry) {
    println!("{:?} - {:?}", SystemTime::now(), dir_entry.path());
    // check if current index exist
    // check if should delete current index
    // parse file
    // save parsed symbols
}

fn parse_file() {
    // create stacks
    // read file lines
    // ...
    // return symbols
}
