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

    /// Use the specified mount options
    #[arg(short, value_name = "options", value_delimiter = ',')]
    options: Vec<String>,
}

impl Arguments {
    pub fn try_parse_from(arguments: Vec<String>) -> Result<Arguments, clap::error::Error> {
        let mut command = Arguments::command();

        command.build();

        let (usage, help) = render_usage_with_help(&command);

        command = command.override_usage(usage);
        command = command.override_help(help);

        let matches = command.try_get_matches_from(arguments)?;
        Arguments::from_arg_matches(&matches)
    }
}

pub fn parse_config(arguments: Vec<String>) -> Result<Arguments, clap::Error> {
    Arguments::try_parse_from(arguments)
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
        let argv = command.map(str::to_string).to_vec();

        let arguments = parse_config(argv).unwrap();

        assert_eq!(arguments.source, Path::new("/images/installer image.iso"));
        assert_eq!(arguments.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn parse_hyphenated_values_as_positionals() {
        let command = ["isomorfs", "-w", "source", "target"];
        let argv = command.map(str::to_string).to_vec();

        let arguments = parse_config(argv).unwrap();

        assert_eq!(arguments.source, Path::new("-w"));
        assert_eq!(arguments.target, Path::new("source"));
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
        let argv = command.map(str::to_string).to_vec();

        let arguments = parse_config(argv).unwrap();

        assert_eq!(arguments.source, Path::new("/images/installer image.iso"));
        assert_eq!(arguments.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn parse_source_and_target() {
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
        let argv = command.map(str::to_string).to_vec();

        let arguments = parse_config(argv).unwrap();

        assert_eq!(arguments.source, Path::new("/images/installer image.iso"));
        assert_eq!(arguments.target, Path::new("/mnt/iso mount"));
    }

    #[test]
    fn reject_missing_arguments() {
        let command = ["INVALID"];
        let arguments = command.map(str::to_string).to_vec();

        let error = parse_config(arguments).unwrap_err();

        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }
}
