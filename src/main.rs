use std::fs::{create_dir_all, File};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

use LTZ_hash_collision::{
    counting_func::{handle_count_single, handle_count_all},
    collision_func::{handle_hash, handle_collision},
};

fn main() -> io::Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        args = input.split_whitespace().map(|s| s.to_string()).collect();
    }
    if args.is_empty() {
        eprintln!("no command provided");
        return Ok(());
    }
    match args[0].as_str() {
        "count" => {
            if args.len() >= 3 && args[1] != "all" {
                let p: u64 = args[1].parse().unwrap_or(0);
                let fname = &args[2];
                handle_count_single(p, fname)?;
            } else if args.len() >= 3 && args[1] == "all" {
                let folder = &args[2];
                handle_count_all(folder)?;
            } else {
                eprintln!("invalid arguments for count");
            }
        }
        "hash" => {
            if args.len() >= 3 {
                let p: u64 = args[1].parse().unwrap_or(0);
                handle_hash(p, &args[2]);
            } else {
                eprintln!("hash requires an argument");
            }
        }
        "collision" => {
            if args.len() >= 2 {
                let p: u64 = args[1].parse().unwrap_or(0);
                handle_collision(p);
            } else {
                eprintln!("collision requires a prime argument");
            }
        }
        _ => {
            eprintln!("unknown command");
        }
    }
    Ok(())
}
