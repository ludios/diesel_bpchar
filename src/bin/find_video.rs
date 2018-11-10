extern crate diesel_bpchar;
extern crate diesel;

use std::env;
use self::diesel_bpchar::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_bpchar::schema::videos::dsl::*;

    let args: Vec<String> = env::args().collect();
    let q = args.get(1).unwrap();

    let connection = establish_connection();
    let results = videos.find(&q)
        .load::<Video>(&connection)
        .expect("Error loading videos");

    println!("Found {} videos", results.len());
}
