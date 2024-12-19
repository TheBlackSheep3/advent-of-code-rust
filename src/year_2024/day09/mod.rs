pub mod error;

mod disk_map;
mod file;

use error::Error;

pub fn get_compacted_filesystem_checksum(input: &str) -> Result<u32, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    pub const SMALL_SAMPLE1: &str = "12345";
    pub const SMALL_SAMPLE2: &str = "90909";
    pub const LARGER_SAMPLE: &str = "2333133121414131402";
}
