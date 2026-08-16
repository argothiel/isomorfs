use std::env;

struct Arguments<'a> {
    source: &'a str,
    target: &'a str,
}

fn parse_config(args: &[String]) -> Arguments<'_> {
    // mount.isomorfs SOURCE TARGET [-sfnv] [-N namespace] [-o options] [-t type.subtype]
    Arguments {source : &args[1], target : &args[2]}
}

fn process(args: &[String]) {
    let config = parse_config(args);

    println!("{} => {}", config.source, config.target);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    process(&args);
}
