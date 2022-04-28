use std::fs::File;
use std::io::{Read, Seek};

pub struct RGB(u8, u8, u8);

pub struct TextToken {
    data: String,
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
    data: Vec<TextToken>,
}

impl Backend {
    pub fn handle_keys(key: char, mod_key: char) {}
    pub fn get_display_data(&mut self, f_w: u32, f_h: u32) -> Vec<Line> {
        let mut line_len:u64;
        let mut i:u64 = 0;
        let mut line_start:u64;
        let mut line_end:u64;
        let mut data = Vec::new();
        loop {
            line_start = self.pos_table[(self.top_line + i) as usize];
            line_end = self.pos_table[(self.top_line + 1 + i) as usize];
            line_len = line_end - line_start;
            if line_len < f_w as u64 {
                 data.push(Line{
                     line_number: self.top_line + i,
                     data: vec![TextToken{
                         color: RGB(255,255,255),
                         data: 
                     }]});
            }
            if i >= f_h as u64 {
                break;
            }
        }
        return data;
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
