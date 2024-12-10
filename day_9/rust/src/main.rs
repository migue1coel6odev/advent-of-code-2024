use std::fs;

use free_space::FreeSpaceV1;
use free_space_v2::FreeSpaceV2;

mod free_space;
mod free_space_v2;

fn main() {
    let input = read_file();
    let mut freespace_v2 = FreeSpaceV2::new(input);
    freespace_v2.decode_disk_space();
    freespace_v2.compact_disk_space();
    freespace_v2.update_filesystem_checksum();
    println!("{}", freespace_v2);
}

fn read_file() -> String {
    String::from_utf8(fs::read("input.txt").unwrap()).unwrap()
}
