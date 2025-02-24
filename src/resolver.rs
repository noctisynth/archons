use crate::{
  apply_opt, to_char_vec,
  types::{Command, CommandMeta, CommandOption},
  utils::{leak_borrowed_str, leak_borrowed_str_or_default, leak_str},
  HashMap,
};

pub(crate) fn resolve_option_args(
  env: napi::Env,
  args: Option<Vec<String>>,
) -> napi::Result<Vec<String>> {
  if let Some(mut args) = args {
    args.remove(0);
    return Ok(args);
  }

  let global = env.get_global()?;
  if let Ok(deno) = global.get_named_property::<napi::JsObject>("Deno") {
    let mut args = vec!["deno".to_string()];
    args.extend(deno.get_named_property::<Vec<String>>("args")?);
    return Ok(args);
  }

  let mut args = global
    .get_named_property::<napi::JsObject>("process")?
    .get_named_property::<Vec<String>>("argv")?;
  args.remove(0);
  Ok(args)
}

pub(crate) fn resolve_command_meta(
  mut clap: clap::Command,
  bin_name: Option<String>,
  meta: &CommandMeta,
) -> clap::Command {
  let name: &'static str = meta.name.as_ref().map_or_else(
    || leak_str(bin_name.expect("bin_name must be provided")),
    |name| leak_borrowed_str(name),
  );
  clap = clap.name(name);

  apply_opt!(clap, meta, leak_borrowed_str(&version) => version);
  apply_opt!(clap, meta, leak_borrowed_str(&about) => about);
  apply_opt!(clap, meta, subcommand_required);

  if meta.styled.unwrap_or(false) {
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

pub(crate) fn resolve_action(action: &Option<String>, r#type: &Option<String>) -> clap::ArgAction {
  let r#type = r#type.as_deref().unwrap_or("option");
  match action.as_deref() {
    Some("set") => clap::ArgAction::Set,
    Some("append") => clap::ArgAction::Append,
    Some("count") => clap::ArgAction::Count,
    Some("store") => clap::ArgAction::SetTrue,
    Some("store_false") => clap::ArgAction::SetFalse,
    None => match r#type {
      "option" | "positional" => clap::ArgAction::Set,
      _ => panic!("Unsupported type: {:?}", r#type),
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

pub(crate) fn resolve_value_hint(value_hint: &str) -> clap::builder::ValueHint {
  match value_hint {
    "any_path" => clap::builder::ValueHint::AnyPath,
    "file" => clap::builder::ValueHint::FilePath,
    "dir" => clap::builder::ValueHint::DirPath,
    "executable" => clap::builder::ValueHint::ExecutablePath,
    "cmd_name" => clap::builder::ValueHint::CommandName,
    "cmd" => clap::builder::ValueHint::CommandString,
    "cmd_with_args" => clap::builder::ValueHint::CommandWithArguments,
    "url" => clap::builder::ValueHint::Url,
    "username" => clap::builder::ValueHint::Username,
    "hostname" => clap::builder::ValueHint::Hostname,
    "email" => clap::builder::ValueHint::EmailAddress,
    _ => panic!("Unsupported value_hint: {:?}", value_hint),
  }
}

pub(crate) fn resolve_num_args(num_args: &str) -> clap::builder::ValueRange {
  if let Ok(n) = num_args.parse::<usize>() {
    return n.into();
  }
  let (start, end) = num_args.split_once("..").expect("Invalid num_args");
  match (start, end) {
    ("", "") => (..).into(),
    ("", end) => {
      if end.starts_with("=") {
        let end: usize = end
          .strip_prefix("=")
          .unwrap()
          .parse()
          .expect("Invalid end of range");
        (..=end).into()
      } else {
        let end: usize = end.parse().expect("Invalid end of range");
        (..end).into()
      }
    }
    (start, "") => {
      let start: usize = start.parse().expect("Invalid start of range");
      (start..).into()
    }
    (start, end) => {
      let start: usize = start.parse().expect("Invalid start of range");
      if end.starts_with("=") {
        let end: usize = end
          .strip_prefix("=")
          .unwrap()
          .parse()
          .expect("Invalid end of range");
        (start..=end).into()
      } else {
        let end: usize = end.parse().expect("Invalid end of range");
        (start..end).into()
      }
    }
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
        arg = arg.action(resolve_action(&opt.action, &opt.r#type));
        if opt.r#type.as_deref() != Some("positional") {
          let long = leak_borrowed_str_or_default(opt.long.as_ref(), name);
          arg = arg.long(long).short(
            leak_borrowed_str_or_default(opt.short.as_ref(), long)
              .chars()
              .next(),
          );
        }
        arg = arg.value_parser(resolve_parser(opt.parser.as_deref(), opt.action.as_deref()));
        apply_opt!(arg, opt, &alias => visible_aliases);
        apply_opt!(arg, opt, &hidden_alias => aliases);
        apply_opt!(arg, opt, to_char_vec!(&short_alias) => short_aliases);
        apply_opt!(arg, opt, to_char_vec!(&hidden_short_alias) => short_aliases);
        apply_opt!(arg, opt, resolve_value_hint(&value_hint) => value_hint);
        apply_opt!(arg, opt, &help);
        apply_opt!(arg, opt, required);
        apply_opt!(arg, opt, default => default_value);
        apply_opt!(arg, opt, default_missing => default_missing_value);
        apply_opt!(arg, opt, resolve_num_args(num_args) => num_args);
        apply_opt!(arg, opt, required_equals => require_equals);
        apply_opt!(arg, opt, hidden => hide);
        apply_opt!(arg, opt, global);
        apply_opt!(arg, opt, exclusive);
        apply_opt!(arg, opt, &conflicts_with => conflicts_with_all);
        apply_opt!(arg, opt, hide_default_value);
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
