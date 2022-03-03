#[macro_use]
extern crate lazy_static;
extern crate clap;

mod crawler;
mod db;
mod http;
mod item;
mod utils;

use clap::{crate_version, App, Arg};

fn main() {
    let matches = App::new("CS:GO item price crawler")
        .version(crate_version!())
        .author("Kirovj")
        .about("Please don't use it illegally, I don't take any responsibility.")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .help("crawl target: buff | yyyp | igxe")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db")
                .short("d")
                .long("database")
                .help("database file name default <target>.db")
                .takes_value(true),
        )
        .get_matches();
    let target = matches.value_of("target").unwrap();
    let db_file = match matches.value_of("db") {
        Some(name) => name.to_string(),
        _ => {
            let mut name = String::from("./data/");
            name.push_str(target);
            name.push_str(".db");
            name
        }
    };
    match crawler::build_crawler(target, db_file.as_str()) {
        Some(crawler) => crawler.run(),
        _ => println!("unknown target {}", target),
    }
}
