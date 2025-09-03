//! a
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
/// a
pub fn generate_hash(selected_hash: usize, image: PathBuf) -> Option<String> {
    match selected_hash {
        1 => Some(hasher::<sha2::Sha512>(&image).unwrap()),
        2 => Some(hasher::<sha2::Sha256>(&image).unwrap()),
        3 => Some(hasher::<sha1::Sha1>(&image).unwrap()),
        4 => Some(hasher::<md5::Md5>(&image).unwrap()),
        5 => Some(hasher::<blake2::Blake2b512>(&image).unwrap()),
        _ => None,
    }
}
fn hasher<H: digest::Digest>(image: &Path) -> io::Result<String> {
    // Use this When File::open_buffered() becomes stable
    /*
    File::open_buffered(image).and_then(|mut file| {
        let mut buffer = [0u8; 8 * 1024];
        let mut hasher = H::new();
        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }

        Ok(format_hash(hasher.finalize().as_slice()))
    })*/

    File::open(image).and_then(move |file| {
        let mut buffer = [0u8; 8 * 1024];
        let mut hasher = H::new();
        let mut file = BufReader::new(file);
        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }

        Ok(format_hash(hasher.finalize().as_slice()))
    })
}
fn format_hash(hash: &[u8]) -> String {
    let mut buf = String::new();
    for b in hash {
        buf.write_fmt(format_args!("{:02x}", *b)).unwrap()
    }
    buf
}
