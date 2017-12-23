extern crate memmem;
extern crate byteorder;

use std::io::{self, Read, Write};
use std::fs::File;
use memmem::{Searcher, TwoWaySearcher};
use byteorder::{BigEndian, ByteOrder};

fn main() {
    println!("What are you searching for?: ");
    let mut find: String = String::new();
    io::stdin().read_line(&mut find).expect("Unable to read line!");
    let find = find.trim_right();
    println!("What are you replacing it with?: ");
    let mut replace: String = String::new();
    io::stdin().read_line(&mut replace);
    let replace = replace.trim_right();
    
    let change: isize = (replace.as_bytes().len() as isize) - (find.as_bytes().len() as isize);
    
    let searcher = TwoWaySearcher::new(find.as_bytes());
    
    for filename in std::env::args().skip(1) {
        let mut in_data: Vec<u8> = Vec::new();
        let mut out_data: Vec<u8> = Vec::new();
        {
            let mut file = File::open(&filename).expect("Unable to open file for reading!");
            file.read_to_end(&mut in_data).expect("Unable to read all data in file!");
        }
        let mut cur_slice: &[u8] = &in_data;
        let mut cur_index: usize = 0;
        while let Some(sub_index) = searcher.search_in(&cur_slice) {
            cur_index += sub_index;
            let (before, middle_after) = cur_slice.split_at(sub_index - 2);
            let cur_size = BigEndian::read_u16(middle_after);
            let new_size = ((cur_size as isize) + change) as u16;
            let mut new_size_buffer: [u8; 2] = [0; 2];
            BigEndian::write_u16(&mut new_size_buffer, new_size);
            let (middle, after) = middle_after.split_at(find.as_bytes().len() + 2);
            cur_slice = after;
            
            out_data.extend_from_slice(before);
            out_data.extend_from_slice(&new_size_buffer);
            out_data.extend_from_slice(replace.as_bytes());
        }
        out_data.extend_from_slice(&cur_slice);
        
        let mut file = File::create(&filename).expect("Unable to open file for writing!");
        file.write_all(&out_data).expect("Unable to write result data!");
    }
}
