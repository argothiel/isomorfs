use std::env;
use std::path::PathBuf;

struct Arguments {
    source: PathBuf,
    target: PathBuf,
}

fn parse_config(args: Vec<String>) -> Arguments {
    // mount.isomorfs SOURCE TARGET [-sfnv] [-N namespace] [-o options] [-t type.subtype]
    let mut args_iter = args.into_iter();

    let _program = args_iter.next();
    let source = args_iter.next();
    let target = args_iter.next();

    Arguments {
        source: source.unwrap().into(),
        target: target.unwrap().into(),
    }
}

fn process(args: Vec<String>) {
    let config = parse_config(args);

    println!("{} => {}", config.source.display(), config.target.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    process(args);
}

#[cfg(test)]
mod tests {
    use crate::parse_config;
    use std::path::Path;

    #[test]
    fn parses_source_and_target_paths() {
        let command = [
            "mount.isomorfs",
            "/images/installer image.iso",
            "/mnt/iso mount",
        ];
        let args = command.map(str::to_string).to_vec();

        let config = parse_config(args);

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn ignore_invalid_arguments() {
        let command = [
            "INVALID",
            "/images/installer image.iso",
            "/mnt/iso mount",
            "foo",
            "-bar",
        ];
        let args = command.map(str::to_string).to_vec();

        let config = parse_config(args);

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn ignore_options() {
        let command = [
            "mount.isomorfs",
            "/images/installer image.iso",
            "/mnt/iso mount",
            "-s",
            "-f",
            "-n",
            "-v",
            "-N",
            "myns",
            "-o",
            "ro,nodev",
            "-t",
            "isomorfs.subtype",
        ];
        let args = command.map(str::to_string).to_vec();

        let config = parse_config(args);

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value"]
    fn panic_at_wrong_number_of_arguments() {
        let command = ["INVALID"];
        let args = command.map(str::to_string).to_vec();

        parse_config(args);
    }
}
