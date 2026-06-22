use hex;
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
    let mut hasher = Sm3::new();
    // 改成 64KB
    let mut bf = [0u8; 64 * 1024];
    loop {
        let bytes_read = file.read(&mut bf)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&bf[..bytes_read]);
    }
    let result = hasher.finalize();
    let sm3_hash = format!("{}", hex::encode(result));
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
