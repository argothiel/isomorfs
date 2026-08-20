use clap::builder::Str;
use clap::{Arg, Command, Id};

pub fn render_usage_with_help(command: &Command) -> (String, String) {
    HelpRenderer::new(command).render_usage_with_help()
}

const SHORT_PLACEHOLDER: &str = "    ";

struct SplitArguments<'a> {
    bin: &'a str,
    positionals: Vec<&'a clap::Arg>,
    short_options: Vec<char>,
    options_with_value: Vec<&'a clap::Arg>,
    all_options: Vec<&'a clap::Arg>,
}

struct HelpRenderer<'a> {
    _command: &'a Command,
    arguments: SplitArguments<'a>,
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
            self.all_options.push(arg);
        }
    }
}

impl<'a> HelpRenderer<'a> {
    fn new(command: &'a Command) -> Self {
        HelpRenderer {
            _command: command,
            arguments: get_split_arguments(command),
        }
    }

    fn render_usage_with_help(&self) -> (String, String) {
        let usage = render_usage(&self.arguments);

        let arguments_help = self.render_table_rows(&self.arguments.positionals);
        let all_options_help = self.render_table_rows(&self.arguments.all_options);
        let help = format!(
            "Usage: {usage}

Arguments:
{arguments_help}

Options:
{all_options_help}\n"
        );

        (usage, help)
    }

    fn render_short(&self, argument: &Arg) -> String {
        if let Some(s) = argument.get_short() {
            return format!("-{s}");
        }
        if argument.get_long().is_some() {
            return SHORT_PLACEHOLDER.to_owned();
        }
        String::new()
    }

    fn render_long(&self, argument: &Arg) -> String {
        if let Some(l) = argument.get_long() {
            if argument.get_short().is_some() {
                return ", --".to_string() + l;
            }
            return "--".to_string() + l;
        }
        String::new()
    }

    fn render_suffix(&self, argument: &Arg) -> String {
        if argument.is_positional() {
            return argument.get_id().to_string();
        }
        if argument.get_num_args().unwrap_or_default().takes_values() {
            let names = argument.get_value_names().unwrap();
            return " ".to_string()
                + names
                    .iter()
                    .map(Str::as_str)
                    .collect::<Vec<_>>()
                    .join(" ")
                    .as_str();
        }
        " ".to_string()
    }

    fn render_description(&self, argument: &Arg) -> String {
        argument.get_help().unwrap_or_default().to_string()
    }

    fn render_label(&self, argument: &Arg) -> String {
        let short = self.render_short(argument);
        let long = self.render_long(argument);
        let suffix = self.render_suffix(argument);
        format!("  {short}{long}{suffix}")
    }

    fn render_row(&self, argument: &Arg, length: usize) -> String {
        format!(
            "{:length$}  {}",
            self.render_label(argument),
            self.render_description(argument)
        )
    }

    fn render_table_rows(&self, arguments: &[&Arg]) -> String {
        if arguments.is_empty() {
            return "".to_string();
        }
        let max_length = arguments
            .iter()
            .map(|arg| self.render_label(arg).len())
            .max()
            .unwrap();

        let rendered_rows: Vec<String> = arguments
            .iter()
            .map(|row| self.render_row(row, max_length))
            .collect();
        rendered_rows.join("\n")
    }
}

fn render_usage_value_option(arg: &Arg) -> String {
    let short = arg.get_short().unwrap();

    let value = arg
        .get_value_names()
        .and_then(<[Str]>::first)
        .map(Str::as_str)
        .unwrap_or("VALUE");
    format!("[-{short} {value}]")
}

fn get_split_arguments(command: &Command) -> SplitArguments<'_> {
    let mut split_arguments = SplitArguments {
        bin: "",
        short_options: Vec::new(),
        options_with_value: Vec::new(),
        positionals: Vec::new(),
        all_options: Vec::new(),
    };

    split_arguments.bin = command.get_bin_name().unwrap_or(command.get_name());
    for arg in command.get_arguments() {
        split_arguments.add(arg)
    }
    split_arguments
}

fn render_usage(split_arguments: &SplitArguments) -> String {
    let SplitArguments {
        bin,
        short_options,
        options_with_value,
        positionals,
        all_options: _,
    } = split_arguments;

    let joined_flags: String = short_options.iter().collect();
    let joined_options: String = options_with_value
        .iter()
        .copied()
        .map(render_usage_value_option)
        .collect::<Vec<_>>()
        .join(" ");
    let joined_positionals: String = positionals
        .iter()
        .copied()
        .map(Arg::get_id)
        .map(Id::as_str)
        .collect::<Vec<_>>()
        .join(" ");

    format!("{bin} {joined_positionals} [-{joined_flags}] {joined_options}")
}
