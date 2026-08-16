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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_source_and_target_paths() {
        let command = [
            "mount.isomorfs",
            "/images/installer image.iso",
            "/mnt/iso mount",
        ];
        let args = command.map(str::to_string);

        let config = parse_config(&args);

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
        let args = command.map(str::to_string);

        let config = parse_config(&args);

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
        let args = command.map(str::to_string);

        let config = parse_config(&args);

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn panic_at_wrong_number_of_arguments() {
        let command = ["INVALID"];
        let args = command.map(str::to_string);

        parse_config(&args);
    }
}
