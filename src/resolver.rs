use std::collections::HashMap;

use crate::{
  types::{Command, CommandMeta, CommandOption},
  utils::{leak_borrowed_str, leak_borrowed_str_or_default, leak_str},
};

pub(crate) fn resolve_option_args(
  env: napi::Env,
  args: Option<Vec<String>>,
) -> napi::Result<Vec<String>> {
  let mut args = match args {
    Some(args) => args,
    None => {
      let mut args = std::env::args().collect::<Vec<String>>();
      if args.is_empty() {
        let process: napi::JsObject = env.get_global()?.get_named_property("process")?;
        let argv: Vec<String> = process.get_named_property("argv")?;
        args = argv;
      }
      args
    }
  };
  args.remove(0); // remove `node.exe`
  Ok(args)
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

pub(crate) fn resolve_action(action: &Option<String>, type_: &Option<String>) -> clap::ArgAction {
  let type_ = type_.as_deref().unwrap_or("option");
  match action.as_deref() {
    Some("set") => clap::ArgAction::Set,
    Some("append") => clap::ArgAction::Append,
    Some("count") => clap::ArgAction::Count,
    Some("store") => clap::ArgAction::SetTrue,
    Some("store_false") => clap::ArgAction::SetFalse,
    None => match type_ {
      "option" => clap::ArgAction::SetTrue,
      "positional" => clap::ArgAction::Set,
      _ => panic!("Unsupported type: {:?}", type_),
    },
    _ => panic!("Unsupported action: {:?}", action),
  }
}

pub(crate) fn resolve_parser(
  parser: Option<&str>,
  action: Option<&str>,
) -> clap::builder::ValueParser {
  match parser {
    Some("string") => clap::builder::ValueParser::string(),
    Some("number") => clap::value_parser!(i64).into(),
    Some("boolean") => clap::builder::ValueParser::bool(),
    None => match action {
      Some("store") | Some("store_false") => clap::builder::ValueParser::bool(),
      Some("count") => clap::value_parser!(u64).into(),
      Some("append") => clap::builder::ValueParser::string(),
      Some("set") => clap::builder::ValueParser::string(),
      None => clap::builder::ValueParser::string(),
      _ => panic!("Unsupported action: {:?}", action),
    },
    _ => panic!("Unsupported parser: {:?}", parser),
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
        arg = arg.action(resolve_action(&opt.action, &opt.type_));
        if opt.type_.as_deref() != Some("positional") {
          let long = leak_borrowed_str_or_default(opt.long.as_ref(), name);
          arg = arg.long(long).short(
            leak_borrowed_str_or_default(opt.short.as_ref(), long)
              .chars()
              .next(),
          );
        }
        arg = arg.value_parser(resolve_parser(opt.parser.as_deref(), opt.action.as_deref()));
        if let Some(alias) = &opt.alias {
          arg = arg.visible_aliases(alias);
        }
        if let Some(hidden_alias) = &opt.hidden_alias {
          arg = arg.aliases(hidden_alias);
        }
        if let Some(short_alias) = &opt.short_alias {
          let short_alias = short_alias
            .iter()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>();
          arg = arg.visible_short_aliases(short_alias);
        }
        if let Some(hidden_short_alias) = &opt.hidden_short_alias {
          let hidden_short_alias = hidden_short_alias
            .iter()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>();
          arg = arg.short_aliases(hidden_short_alias);
        }
        if let Some(help) = &opt.help {
          arg = arg.help(help);
        }
        if let Some(required) = opt.required {
          arg = arg.required(required);
        }
        if let Some(default) = &opt.default {
          arg = arg.default_value(default);
        }
        if let Some(default_missing) = opt.default_missing {
          arg = arg.default_missing_value(default_missing);
        }
        if let Some(hidden) = opt.hidden {
          arg = arg.hide(hidden);
        }
        if let Some(global) = opt.global {
          arg = arg.global(global);
        }
        if let Some(conflicts_with) = &opt.conflicts_with {
          arg = arg.conflicts_with_all(conflicts_with);
        }
        if let Some(hide_default_value) = opt.hide_default_value {
          arg = arg.hide_default_value(hide_default_value);
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
