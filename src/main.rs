extern crate termion;

use std::env;
use std::fs::File;
use std::io::{Read, Seek};
use termion::{clear, color, cursor, terminal_size};
fn main() {
    let args: Vec<String> = env::args().collect();
    let term_size = terminal_size().unwrap();
    print!("{}{}", clear::All, cursor::Goto(1, 1),);
    let mut f = File::open(&args[1]).expect("Unable to open the specified file");
    print!("Line endings are at {:?}", get_position_table(&mut f));
}

fn get_position_table(file: &mut File) -> Vec<u64> {
    let file_size = file.metadata().unwrap().len();
    let mut pos_table = Vec::new();
    const BUF_SIZE: usize = 4096;
    let mut smol_buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    file.read_exact(&mut smol_buf).expect("Failed to read file");
    while file_size - BUF_SIZE as u64
        > file
            .stream_position()
            .expect("Could not get the stream positon")
    {
        for (i, e) in smol_buf.iter().enumerate() {
            if e == &10u8 {
                pos_table.push(
                    file.stream_position()
                        .expect("Could not get the stream positon")
                        - ((BUF_SIZE - i) as u64),
                )
            }
        }
    }
    let mut data_end = vec![];
    let read_bytes = file
        .read_to_end(&mut data_end)
        .expect("Could not read the last 4096 bytes of the file");
    for (i, e) in data_end.iter().enumerate() {
        if e == &10u8 {
            pos_table.push(
                file.stream_position()
                    .expect("Could not get the stream positon")
                    - ((read_bytes - i) as u64),
            )
        }
    }
    file.rewind()
        .expect("Could not rewind back to the start of the file");
    return pos_table;
}


fn goto(x: usize, y: usize) {
    
}
