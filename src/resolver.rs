use std::collections::HashMap;

use crate::{
  types::{Command, CommandMeta, CommandOption},
  utils::{leak_borrowed_str, leak_borrowed_str_or_default, leak_str},
};

pub(crate) fn resolve_option_args(args: Option<Vec<String>>) -> Vec<String> {
  let mut args = args.unwrap_or(std::env::args().collect());
  args.remove(0); // remove `node.exe`
  args
}

pub(crate) fn resolve_command_meta(
  mut clap: clap::Command,
  bin_name: Option<String>,
  meta: &CommandMeta,
) -> clap::Command {
  let name: &'static str = if let Some(name) = &meta.name {
    leak_borrowed_str(name)
  } else {
    leak_str(bin_name.unwrap())
  };
  clap = clap.name(name);
  if let Some(version) = &meta.version {
    clap = clap.version(leak_borrowed_str(version));
  }
  if let Some(about) = &meta.about {
    clap = clap.about(leak_borrowed_str(about));
  }
  if meta.styled.is_some() && meta.styled.unwrap() {
    use clap::builder::styling;
    let styles = styling::Styles::styled()
      .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
      .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
      .literal(styling::AnsiColor::Cyan.on_default() | styling::Effects::BOLD)
      .placeholder(styling::AnsiColor::Cyan.on_default());
    clap = clap.styles(styles);
  }
  clap
}

pub(crate) fn resolve_action(_type: &str, parser: Option<&str>) -> Option<clap::ArgAction> {
  match _type {
    "positional" | "option" => {
      if parser.is_some() && parser.unwrap().ends_with("[]") {
        Some(clap::ArgAction::Append)
      } else {
        None
      }
    }
    "flag" => match parser {
      Some("bool" | "boolean") | None => Some(clap::ArgAction::SetTrue),
      Some("number") => Some(clap::ArgAction::Count),
      _ => panic!("Invalid flag parser: `{:?}`", parser),
    },
    _ => panic!("Unsupported option type: `{}`", _type),
  }
}

pub(crate) fn resolve_command_options(
  clap: clap::Command,
  meta: &HashMap<String, CommandOption>,
) -> clap::Command {
  clap.args(
    meta
      .iter()
      .map(|(name, opt)| {
        let mut arg = clap::Arg::new(leak_borrowed_str(name));
        arg = arg.action(resolve_action(
          opt._type.as_deref().unwrap_or("option"),
          opt.parser.as_deref(),
        ));
        if opt._type.as_deref() != Some("positional") {
          let long = leak_borrowed_str_or_default(opt.long.as_ref(), name);
          arg = arg.long(long).short(
            leak_borrowed_str_or_default(opt.short.as_ref(), long)
              .chars()
              .next(),
          );
        }
        if let Some(alias) = &opt.alias {
          let alias = alias.iter().map(leak_borrowed_str).collect::<Vec<&str>>();
          arg = arg.visible_aliases(alias);
        }
        if let Some(hidden_alias) = &opt.hidden_alias {
          let hidden_alias = hidden_alias
            .iter()
            .map(leak_borrowed_str)
            .collect::<Vec<&str>>();
          arg = arg.aliases(hidden_alias);
        }
        if let Some(help) = &opt.help {
          arg = arg.help(leak_borrowed_str(help));
        }
        if let Some(required) = opt.required {
          arg = arg.required(required);
        }
        if let Some(default) = &opt.default {
          arg = arg.default_value(leak_borrowed_str(default));
        }
        if let Some(hidden) = opt.hidden {
          arg = arg.hide(hidden);
        }
        arg
      })
      .collect::<Vec<clap::Arg>>(),
  )
}

pub(crate) fn resolve_command(
  mut clap: clap::Command,
  name: String,
  cmd: &Command,
) -> clap::Command {
  clap = resolve_command_meta(clap, Some(name), &cmd.meta);
  clap = resolve_command_options(clap, &cmd.options);
  if let Some(subcommands) = &cmd.subcommands {
    clap = clap.subcommands(
      subcommands
        .iter()
        .map(|(name, sub_cmd)| resolve_command(clap::Command::default(), name.clone(), sub_cmd))
        .collect::<Vec<clap::Command>>(),
    );
  }
  clap
}
