use crate::file_entry::FileEntry;
use crc::Crc;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::{fs, io};

/// Check if a directory is empty.
pub fn is_dir_empty(dir: &Path) -> bool {
    fs::read_dir(dir)
        .ok()
        .map_or(false, |mut entries| entries.next().is_none())
}

/// Generate a unique filename if the file already exists in the destination.
pub fn generate_unique_filename(file: &FileEntry, target_dir: &Path) -> io::Result<String> {
    let (stem, ext) = (file.stem(), file.extension());

    for counter in 1.. {
        let filename = format!("{}_{}{}", stem, counter, ext);
        if !target_dir.join(&filename).exists() {
            return Ok(filename);
        }
    }

    unreachable!()
}

/// Calculate the CRC64 checksum of a file.
pub fn crc64_file_checksum(path: &Path) -> Result<String, Error> {
    static CRC64_HASHER: Crc<u64> = Crc::<u64>::new(&crc::CRC_64_ECMA_182);

    let mut reader = BufReader::new(File::open(path)?);
    let mut digest = CRC64_HASHER.digest();
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        digest.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:016x}", digest.finalize()))
}
