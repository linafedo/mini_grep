use std::env;
use mini_grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    mini_grep::run(args);
}


