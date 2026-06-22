mod arg;
mod calculate;

use crate::arg::args::Args;
use crate::calculate::sm3::calculate_sm3;
use clap::Parser;
use std::time::Instant;
fn main() {
    let instant = Instant::now();
    let args = Args::parse();
    let file = &args.file;
    match calculate_sm3(file) {
        Ok(sm3) => {
            let millis = instant.elapsed().as_millis();
            if millis < 60000 {
                println!("Calculate the time consumed by SM3: {millis} ms");
            } else {
                let secs = millis / 1000;
                println!("Calculate the time consumed by SM3: {secs} s");
            }
            println!("SM3 Hash: {sm3}")
        }
        Err(e) => {
            eprintln!("Error:[{file}] calculate SM3 hash error! {e}");
            return;
        }
    }
}
