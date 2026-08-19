use clap::{CommandFactory, FromArgMatches, Parser};
use std::path::PathBuf;

mod mount_cli;

use self::mount_cli::render_usage_with_help;

// /sbin/mount.suffix spec dir [-sfnv] [-N namespace] [-o options] [-t type.subtype]
#[derive(Parser, Debug)]
#[command(allow_hyphen_values = true, ignore_errors = true)]
pub struct Arguments {
    /// Source .iso image
    pub source: PathBuf,

    /// Target mount point
    pub target: PathBuf,

    /// Tolerate sloppy mount options rather than fail
    #[arg(short)]
    sloppy: bool,

    /// Dry run; do not actually mount
    #[arg(short, long)]
    fake: bool,

    /// Don't write to /etc/mtab
    #[arg(short, long)]
    no_mtab: bool,

    /// Say what is being done
    #[arg(short, long)]
    verbose: bool,

    /// Perform mount in another namespace
    #[arg(short = 'N', long, value_name = "ns")]
    namespace: Option<String>,

    #[arg(short, value_name = "options", value_delimiter = ',')]
    options: Vec<String>,
}

impl Arguments {
    pub fn try_parse_from(args: Vec<String>) -> Result<Arguments, clap::error::Error> {
        let mut command = Arguments::command();
        command.build();

        let (usage, help) = render_usage_with_help(&command);

        command = command.override_usage(usage);
        command = command.override_help(help);

        let matches = command.try_get_matches_from(args)?;
        Arguments::from_arg_matches(&matches)
    }
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
        let command = ["isomorfs", "-w", "source", "target"];
        let args = command.map(str::to_string).to_vec();

        let config = parse_config(args).unwrap();

        assert_eq!(config.source, Path::new("-w"));
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
