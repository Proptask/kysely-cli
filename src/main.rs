use std::fs::{ self, File };
use std::io::prelude::*;

use chrono::Utc;
use clap::Parser;

fn add_migration(name: &str) {
    // Get the current date and time in ISO format
    let now = Utc::now();
    let date_str = now.format("%Y-%m-%dT%H%M%S%.3fZ").to_string();

    // Create the file name
    let file_name = format!("{}_{}.ts", date_str, name);

    // Create a new file with the specified name
    let mut file = match File::create(&file_name) {
        Err(why) => panic!("Couldn't create {}: {}", &file_name, why),
        Ok(file) => file,
    };

    let content = fs::read_to_string("./templates/migration.ts");

    // Write the content to the file
    match file.write_all(content.unwrap().as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", &file_name, why),
        Ok(_) => println!("Succesfully created kysely migration {}", &file_name),
    };
}

/// This cli tool is to simplify the process of handling git branches for deployment
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Args {
    /// Merges the branch to the target branch and switches back to the default branch development
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    add_migration(&args.name);
}
