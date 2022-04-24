fn goto(x: usize, y: usize) {
    print!("{}[{};{}H", 27 as char, y, x);
}

fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn init_frontend() {
    goto(0, 0);
    clear();
}
