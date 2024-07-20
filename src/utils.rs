use std::collections::HashMap;

use napi::{JsObject, Result};

use crate::types::{Command, CommandMeta, CommandOption};

pub(crate) fn leak_str(s: String) -> &'static str {
  s.leak()
}

pub(crate) fn leak_borrowed_str_or_default(s: Option<&String>, default: &str) -> &'static str {
  s.map_or_else(|| leak_str(default.to_string()), |s| leak_str(s.clone()))
}

pub(crate) fn leak_borrowed_str(s: impl Into<String>) -> &'static str {
  Into::<String>::into(s).leak()
}

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

pub(crate) fn merge_args_matches(
  parsed_args: &mut JsObject,
  cmd: &Command,
  matches: &clap::ArgMatches,
) -> Result<()> {
  for id in matches.ids() {
    let cmd = &cmd;
    let opts = cmd
      .options
      .iter()
      .find(|&(name, _)| name == id)
      .map(|(_, t)| t)
      .unwrap();
    match opts._type.as_deref().unwrap_or("option") {
      "option" => {
        parsed_args.set(id, matches.get_one::<String>(id.as_str()))?;
      }
      "flag" => {
        parsed_args.set(id, matches.get_flag(id.as_str()))?;
      }
      "positional" => {
        parsed_args.set(id, matches.get_one::<String>(id.as_str()))?;
      }
      _ => panic!("Unsupported option type"),
    }
  }
  Ok(())
}
