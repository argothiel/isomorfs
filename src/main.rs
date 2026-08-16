use std::env;
use std::path::Path;

struct Arguments<'a> {
    source: &'a Path,
    target: &'a Path,
}

fn parse_config(args: &[String]) -> Arguments<'_> {
    // mount.isomorfs SOURCE TARGET [-sfnv] [-N namespace] [-o options] [-t type.subtype]
    Arguments {
        source: Path::new(&args[1]),
        target: Path::new(&args[2]),
    }
}

fn process(args: &[String]) {
    let config = parse_config(args);

    println!("{} => {}", config.source.display(), config.target.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    process(&args);
}
