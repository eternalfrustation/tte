
use std::io::{Read, Seek};
use std::fs::File;

pub struct RGB(u8, u8, u8);

pub struct Color_component {
    data_size: u64,
    data_pos: u64,
    color: RGB,
}

pub struct Backend {
    cursor_pos: u64,
    top_line: u64,
    pos_table: Vec<u64>,
    file: File,
}

pub struct Line {
    line_number: u64,
    color_table: Vec<Color_component>,
    data: Vec<Vec<u8>>,
}

impl Backend {
    pub fn handle_keys(key: char, mod_key: char) {
        
    }
    pub fn get_display_data(f_w: i32, f_h: i32) {
        
    }
    
}

pub fn get_position_table(file: &mut File) -> Vec<u64> {
    let file_size = file.metadata().unwrap().len();
    let mut pos_table = Vec::new();
    const BUF_SIZE: usize = 500000;
    let mut data_end = vec![];
    if file_size < BUF_SIZE as u64 {
        // In case the file is smaller than the buffersize
        let read_bytes = file
            .read_to_end(&mut data_end)
            .expect("Could not read the file");
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
    } else {
        // In case the file size is greater than the size of the buffer
        let mut smol_buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        file.read_exact(&mut smol_buf).expect("Failed to read file");
        while file_size - BUF_SIZE as u64
            > file
                .stream_position()
                .expect("Could not get the stream positon")
        {
            file.read_exact(&mut smol_buf).expect("Failed to read file");
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
        // In case the file size is not a multiple of the buffer size
        // and some data is leftover
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
    }
    return pos_table;
}
// This returns an array of LINES f_w and f_h are frontend's width and height
//
pub fn get_file_data() {}
