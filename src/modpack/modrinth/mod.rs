pub mod structs;

use serde_json::error::Result;
use std::io::{Read, Seek};
use structs::Metadata;
use zip::{result::ZipResult, ZipArchive};

/// Read the `input`'s metadata file to a string
pub fn read_metadata_file(input: impl Read + Seek) -> ZipResult<String> {
    let mut buffer = String::new();
    ZipArchive::new(input)?
        .by_name("modrinth.index.json")?
        .read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Deserialise the given `input` into metadata
pub fn deser_metadata(input: &str) -> Result<Metadata> {
    serde_json::from_str(input)
}
