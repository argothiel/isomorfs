use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = &args[1];
    let target = &args[2];

    println!("{source} => {target}");
}
