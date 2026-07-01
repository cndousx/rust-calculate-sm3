mod arg;
mod calculate;

use crate::arg::args::Args;
use crate::calculate::sm3::calculate_sm3;
use clap::Parser;
fn main() {
    let args = Args::parse();
    let file = &args.file;
    match calculate_sm3(file) {
        Ok(sm3) => {
            println!("SM3:{sm3}")
        }
        Err(e) => {
            eprintln!("Error:[{file}] calculate SM3 hash error! {e}");
            return;
        }
    }
}
