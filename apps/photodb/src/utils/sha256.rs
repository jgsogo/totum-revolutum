use anyhow::Result;
use camino::Utf8Path;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

/// Computes the sha256 digest of the given buffer.
///
/// Credit: https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

/// Computes sha256 and returns it as lowercase string
pub fn sha256_string<R: Read>(reader: R) -> Result<String> {
    let digest = sha256_digest(reader)?;
    Ok(HEXLOWER.encode(digest.as_ref()))
}

pub fn sha256_string_from_file(file: &Utf8Path) -> Result<String> {
    let input = File::open(file)?;
    let reader = BufReader::new(input);
    sha256_string(reader)
}
