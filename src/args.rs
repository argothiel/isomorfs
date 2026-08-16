use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(allow_hyphen_values = true, ignore_errors = true)]
pub struct Arguments {
    pub source: PathBuf,
    pub target: PathBuf,
}

pub fn parse_config(args: Vec<String>) -> Result<Arguments, clap::Error> {
    Arguments::try_parse_from(args)
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

        let config = parse_config(args).unwrap();

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn parse_hyphenated_values_as_positionals() {
        let command = ["isomorfs", "-v", "source", "target"];
        let args = command.map(str::to_string).to_vec();

        let config = parse_config(args).unwrap();

        assert_eq!(config.source, Path::new("-v"));
        assert_eq!(config.target, Path::new("source"));
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

        let config = parse_config(args).unwrap();

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

        let config = parse_config(args).unwrap();

        assert_eq!(config.source, Path::new("/images/installer image.iso"));
        assert_eq!(config.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn reject_wrong_number_of_arguments() {
        let command = ["INVALID"];
        let args = command.map(str::to_string).to_vec();

        let error = parse_config(args).unwrap_err();

        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }
}
