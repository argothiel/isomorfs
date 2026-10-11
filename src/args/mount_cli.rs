use clap::builder::{Str, StyledStr};
use clap::{Arg, Command, Id};
use std::fmt::Write;
use std::iter;

pub fn render_usage_with_help(command: &Command) -> (StyledStr, StyledStr) {
    let (usage, help) = HelpRenderer::new(command).render_usage_with_help();
    (usage.into(), help.into())
}

const SHORT_PLACEHOLDER: &str = "    ";

struct StylePreservingStr(StyledStr);

struct SplitArguments<'a> {
    bin: &'a str,
    positionals: Vec<&'a clap::Arg>,
    short_options: Vec<char>,
    options_with_value: Vec<&'a clap::Arg>,
    all_options: Vec<&'a clap::Arg>,
}

struct HelpRenderer<'a> {
    command: &'a Command,
    arguments: SplitArguments<'a>,
}

impl std::fmt::Display for StylePreservingStr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let padding = formatter
            .width()
            .unwrap_or(0)
            .saturating_sub(self.display_width());
        write!(formatter, "{}{:padding$}", self.0.ansi(), "")
    }
}

impl Write for StylePreservingStr {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        Write::write_str(&mut self.0, s)
    }
}

impl From<String> for StylePreservingStr {
    fn from(value: String) -> Self {
        Self(StyledStr::from(value))
    }
}

impl From<StylePreservingStr> for StyledStr {
    fn from(value: StylePreservingStr) -> Self {
        value.0
    }
}

impl StylePreservingStr {
    fn new() -> StylePreservingStr {
        StylePreservingStr(StyledStr::new())
    }

    fn display_width(&self) -> usize {
        self.0.to_string().chars().count()
    }
}

impl<'a> SplitArguments<'a> {
    fn add(&mut self, arg: &'a Arg) {
        if arg.is_positional() {
            self.positionals.push(arg);
            return;
        }

        let Some(short) = arg.get_short() else {
            panic!("invalid argument: {}", arg);
        };

        if arg.get_action().takes_values() {
            self.options_with_value.push(arg);
            self.all_options.push(arg);
            return;
        }

        if short != 'h' {
            self.short_options.push(short);
        }
        self.all_options.push(arg);
    }
}

impl<'a> HelpRenderer<'a> {
    fn new(command: &'a Command) -> Self {
        HelpRenderer {
            command,
            arguments: get_split_arguments(command),
        }
    }

    fn render_usage_with_help(&self) -> (StylePreservingStr, StylePreservingStr) {
        let usage_text = self.render_usage_without_title();

        let arguments_help = self.render_table_rows(&self.arguments.positionals);
        let all_options_help = self.render_table_rows(&self.arguments.all_options);

        let style = self.command.get_styles();
        let usage = style.get_usage();
        let header = style.get_header();

        let mut help = StylePreservingStr::new();
        let _ = write!(help, "{usage}Usage{usage:#}: {usage_text}");

        if !self.arguments.positionals.is_empty() {
            let _ = write!(
                help,
                "

{header}Arguments{header:#}:
{arguments_help}"
            );
        }
        if !self.arguments.all_options.is_empty() {
            let _ = write!(
                help,
                "

{header}Options{header:#}:
{all_options_help}\n"
            );
        }

        (usage_text, help)
    }

    fn render_short(&self, argument: &Arg) -> StylePreservingStr {
        let literal = self.command.get_styles().get_literal();

        if let Some(s) = argument.get_short() {
            return StylePreservingStr::from(format!("{literal}-{s}{literal:#}"));
        }
        if argument.get_long().is_some() {
            return StylePreservingStr::from(SHORT_PLACEHOLDER.to_owned());
        }
        StylePreservingStr::new()
    }

    fn render_long(&self, argument: &Arg) -> StylePreservingStr {
        let literal = self.command.get_styles().get_literal();

        if let Some(l) = argument.get_long() {
            if argument.get_short().is_some() {
                return StylePreservingStr::from(format!(", {literal}--{l}{literal:#}"));
            }
            return StylePreservingStr::from("--".to_string() + l);
        }
        StylePreservingStr::new()
    }

    fn render_suffix(&self, argument: &Arg) -> StylePreservingStr {
        let placeholder = self.command.get_styles().get_placeholder();

        if argument.is_positional() {
            let positional = argument.get_id().to_string();
            return StylePreservingStr::from(format!("{placeholder}{positional}{placeholder:#}"));
        }

        if argument.get_action().takes_values() {
            let args_range = argument.get_num_args().unwrap_or_default();
            let names = argument.get_value_names().unwrap_or_default();
            let min_values = args_range.min_values();
            let max_values = args_range.max_values();

            let required = names
                .iter()
                .map(Str::as_str)
                .chain(iter::repeat("VALUE"))
                .take(min_values)
                .map(|arg| format!("{placeholder}{arg}{placeholder:#}"));
            let optional = names
                .iter()
                .skip(min_values)
                .map(Str::as_str)
                .chain(iter::repeat("VALUE"))
                .take(max_values - min_values)
                .map(|arg| format!("[{placeholder}{arg}{placeholder:#}]"));

            let suffixes = required.chain(optional).collect::<Vec<_>>().join(" ");

            return StylePreservingStr::from(format!(" {suffixes}"));
        }
        StylePreservingStr::from("".to_string())
    }

    fn render_description(&self, argument: &Arg) -> StylePreservingStr {
        StylePreservingStr::from(argument.get_help().unwrap_or_default().to_string())
    }

    fn render_label(&self, argument: &Arg) -> StylePreservingStr {
        let short = self.render_short(argument);
        let long = self.render_long(argument);
        let suffix = self.render_suffix(argument);
        StylePreservingStr::from(format!("  {short}{long}{suffix}"))
    }

    fn render_row(&self, argument: &Arg, label_width: usize) -> StylePreservingStr {
        StylePreservingStr::from(format!(
            "{:label_width$}  {}",
            self.render_label(argument),
            self.render_description(argument)
        ))
    }

    fn render_table_rows(&self, arguments: &[&Arg]) -> StylePreservingStr {
        if arguments.is_empty() {
            return StylePreservingStr::from("".to_string());
        }
        let max_length = arguments
            .iter()
            .map(|arg| self.render_label(arg).display_width())
            .max()
            .unwrap();

        let rendered_rows: Vec<String> = arguments
            .iter()
            .map(|row| self.render_row(row, max_length).to_string())
            .collect();
        StylePreservingStr::from(rendered_rows.join("\n"))
    }

    fn render_usage_without_title(&self) -> StylePreservingStr {
        let SplitArguments {
            bin,
            short_options,
            options_with_value,
            positionals,
            all_options: _,
        } = &self.arguments;

        let literal = self.command.get_styles().get_literal();
        let placeholder = self.command.get_styles().get_placeholder();

        let joined_flags: String = short_options.iter().collect();
        let joined_options: String = options_with_value
            .iter()
            .copied()
            .map(|option| self.render_usage_value_option(option))
            .map(|option| StylePreservingStr::to_string(&option))
            .collect::<Vec<_>>()
            .join(" ");
        let joined_positionals: String = positionals
            .iter()
            .copied()
            .map(Arg::get_id)
            .map(Id::as_str)
            .map(|positional| format!("{placeholder}{positional}{placeholder:#}"))
            .collect::<Vec<_>>()
            .join(" ");

        let mut writer = StylePreservingStr::new();
        let _ = write!(writer, "{literal}{bin}{literal:#}");

        if !joined_positionals.is_empty() {
            let _ = write!(writer, " {joined_positionals}");
        }
        if !joined_flags.is_empty() {
            let _ = write!(writer, " [{literal}-{joined_flags}{literal:#}]");
        }
        if !joined_options.is_empty() {
            let _ = write!(writer, " {joined_options}");
        }
        writer
    }

    fn render_usage_value_option(&self, arg: &Arg) -> StylePreservingStr {
        let literal = self.command.get_styles().get_literal();
        let placeholder = self.command.get_styles().get_placeholder();

        let short = arg
            .get_short()
            .expect("all the non-positional arguments must support a short flag");

        let value = arg
            .get_value_names()
            .and_then(<[Str]>::first)
            .map(Str::as_str)
            .unwrap_or("VALUE");
        StylePreservingStr::from(format!(
            "[{literal}-{short}{literal:#} {placeholder}{value}{placeholder:#}]"
        ))
    }
}

fn get_split_arguments(command: &Command) -> SplitArguments<'_> {
    let mut split_arguments = SplitArguments {
        bin: command.get_bin_name().unwrap_or(command.get_name()),
        short_options: Vec::new(),
        options_with_value: Vec::new(),
        positionals: Vec::new(),
        all_options: Vec::new(),
    };

    for arg in command.get_arguments() {
        split_arguments.add(arg)
    }
    split_arguments
}

#[cfg(test)]
mod tests {
    use super::render_usage_with_help;
    use anstyle::AnsiColor;
    use clap::builder::Styles;
    use clap::{Command, arg};

    #[test]
    fn renders_usage_with_help_with_colors() {
        let header = AnsiColor::Yellow.on(AnsiColor::Green);
        let usage = AnsiColor::Green.on(AnsiColor::Red);
        let literal = AnsiColor::Red.on(AnsiColor::Blue);
        let placeholder = AnsiColor::Blue.on(AnsiColor::Yellow);

        let styles = Styles::styled()
            .header(header)
            .usage(usage)
            .literal(literal)
            .placeholder(placeholder);

        let c = Command::new("abc")
            .styles(styles)
            .args([arg!(verbose: -v "verbose desc")]);
        let (rendered_usage, rendered_help) = render_usage_with_help(&c);

        assert_eq!(
            rendered_usage,
            format!("{literal}abc{literal:#} [{literal}-v{literal:#}]").into()
        );
        assert_eq!(
            rendered_help,
            format!(
                "{usage}Usage{usage:#}: {literal}abc{literal:#} [{literal}-v{literal:#}]

{header}Options{header:#}:
  {literal}-v{literal:#}  verbose desc
"
            )
            .into()
        );
    }
}
