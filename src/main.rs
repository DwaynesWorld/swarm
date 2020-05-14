use globwalk::*;
use rayon::prelude::*;
use rocksdb::*;
use std::result::Result;
use std::time::{Instant, SystemTime};

const PATTERNS: &'static [&'static str] =
    &["*.{vw,rv,sl,dg,src,dd,pkg,mod,cls,cls,bpo,rpt,mnu,cal,con}"];

fn main() {
    println!("{} cores", num_cpus::get());
    println!("{} cpus", num_cpus::get_physical());

    let base_dir = "/Users/KT/Documents/HeavyBid";
    let db = DB::open_default("/temp/temp.db").unwrap();
    let _ = build_index(base_dir, &db);
}

fn build_index(base_dir: &str, database: &DB) {
    let now = Instant::now();

    GlobWalkerBuilder::from_patterns(base_dir, PATTERNS)
        .case_insensitive(true)
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .for_each(move |e| index_file(e, database));

    println!("{:?}", now.elapsed())
}

fn index_file(dir_entry: &DirEntry, database: &DB) {
    println!("{:?} - {:?}", SystemTime::now(), dir_entry.path());
    let path = dir_entry.path();
    let modified = dir_entry.metadata().unwrap().modified().unwrap();
    database.put(path, b"yay!!").unwrap()
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
