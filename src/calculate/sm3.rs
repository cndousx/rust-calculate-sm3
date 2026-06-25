use hex;
use indicatif::{ProgressBar, ProgressStyle};
use sm3::{Digest, Sm3};
use std::fs::{File, metadata};
use std::io::{Error, Read};
use std::path::Path;

fn file_exists<P>(path: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    match metadata(&path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn calculate_sm3_hash(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let total = file.metadata()?.len();
    let mut hasher = Sm3::new();
    let pb = ProgressBar::new(total);
    pb.set_style(
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({percent}%) ETA:{eta}{msg}"
    )
    .unwrap()
    .progress_chars("█▓▒░")
    .tick_chars("🕛🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚"),);
    // pb.set_message(format!("{}", file_path));
    let mut bf = [0u8; 64 * 1024];
    let mut processed: u64 = 0;
    loop {
        let bytes_read = file.read(&mut bf)?;

        if bytes_read == 0 {
            break;
        }
        hasher.update(&bf[..bytes_read]);
        processed += bytes_read as u64;
        pb.set_position(processed); // 推进进度
    }
    let result = hasher.finalize();
    let sm3_hash = format!("{}", hex::encode(result));
    pb.finish_with_message(format!("\r\nSM3:{}", sm3_hash));
    Ok(sm3_hash)
}

pub fn calculate_sm3(file_path: &str) -> Result<String, Error> {
    match file_exists(file_path) {
        Err(e) => Err(e),
        Ok(exist) => {
            if exist {
                match calculate_sm3_hash(file_path) {
                    Ok(sm3) => Ok(sm3),
                    Err(e) => Err(e),
                }
            } else {
                Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("[{file_path}]file is not exists."),
                ))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 同一个文件，release生成的可执行文件会快很多
        //  1.4G的文件 单测和cargo run需要45秒
        //  而relseae生成的可执行文件，只需要两三秒
        match calculate_sm3("README.md") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error:calculate SM3 hash error! {e}");
                return;
            }
        }
    }
}
