use crate::backend::get_position_table;
use crate::frontend::init_frontend;
use std::env;
use std::fs::File;
mod backend;
mod frontend;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("usage: tte <filename>\n");
        return;
    }
    let mut f = File::open(&args[1]).expect("Unable to open the specified file");
    print!("Line endings are at {:?}", get_position_table(&mut f));
    init_frontend();
}
