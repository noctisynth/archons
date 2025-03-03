use napi::{Env, JsNull, JsObject};

use crate::types::{Command, Context};
use crate::HashMap;

const ISSUE_LINK: &str = "https://github.com/noctisynth/archons/issues";

#[inline]
pub(crate) fn leak_str<'a>(s: String) -> &'a str {
  s.leak()
}

#[inline]
pub(crate) fn leak_borrowed_str<'a>(s: &str) -> &'a str {
  unsafe { std::mem::transmute(s) }
}

#[inline]
pub(crate) fn leak_borrowed_str_or_default<'a>(s: Option<&String>, default: &str) -> &'a str {
  s.map_or_else(|| leak_borrowed_str(default), |s| leak_borrowed_str(s))
}

#[inline(always)]
pub fn as_usize(num: u32) -> usize {
  num as usize
}

#[inline]
pub fn wrap_string_formatter<'a>(formatter: napi::JsFunction) -> &'a (dyn Fn(&str) -> String + 'a) {
  let formatter = Box::new(move |value: &str| {
    let res: String = formatter.call1(value).unwrap_or(value.to_string());
    res
  });
  Box::leak(formatter)
}

#[inline]
pub fn wrap_bool_formatter<'a>(formatter: napi::JsFunction) -> &'a (dyn Fn(bool) -> String + 'a) {
  let formatter = Box::new(move |value: bool| {
    let res: String = formatter.call1(value).unwrap_or(value.to_string());
    res
  });
  Box::leak(formatter)
}

#[inline]
pub fn wrap_bool_parser<'a>(
  parser: napi::JsFunction,
) -> &'a (dyn Fn(&str) -> Result<bool, ()> + 'a) {
  let parser = Box::new(move |value: &str| {
    let res: bool = parser.call1(value).unwrap_or(false);
    Ok(res)
  });
  Box::leak(parser)
}

pub(crate) fn merge_args_matches(
  parsed_args: &mut JsObject,
  args: &[&clap::Arg],
  options: &HashMap<String, &'static str>,
  matches: &clap::ArgMatches,
) -> napi::Result<()> {
  for id in matches.ids() {
    let action = args
      .iter()
      .find(|arg| arg.get_id() == id)
      .map(|arg| arg.get_action())
      .unwrap_or_else(|| {
        panic!(
          "Argument not found when merging matches, this is likely a internal bug.\n
          If you convinced this is a bug, report it at: {}",
          ISSUE_LINK
        )
      });
    let option: &str = options.get(id.as_str()).unwrap();
    match action {
      clap::ArgAction::Set => match option {
        "string" => {
          parsed_args.set(id, matches.get_one::<String>(id.as_str()).unwrap())?;
        }
        "number" => {
          parsed_args.set(id, *matches.get_one::<i64>(id.as_str()).unwrap())?;
        }
        _ => panic!("Invalid option type: {}", option),
      },
      clap::ArgAction::SetTrue | clap::ArgAction::SetFalse => {
        parsed_args.set(id, matches.get_flag(id.as_str()))?;
      }
      clap::ArgAction::Count => {
        parsed_args.set(id, matches.get_count(id.as_str()))?;
      }
      clap::ArgAction::Append => match option {
        "string" => {
          parsed_args.set(
            id,
            matches
              .get_many::<String>(id.as_str())
              .unwrap_or_default()
              .map(|v| v.as_str())
              .collect::<Vec<_>>(),
          )?;
        }
        "number" => {
          parsed_args.set(
            id,
            matches
              .get_many::<f64>(id.as_str())
              .unwrap_or_default()
              .copied()
              .collect::<Vec<_>>(),
          )?;
        }
        _ => panic!("Invalid option type: {}", option),
      },
      _ => panic!("Unsupported argument action: {:?}", action),
    }
  }
  Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_arguments_inner<'arg>(
  env: Env,
  mut parsed_args: JsObject,
  clap: &'arg clap::Command,
  cmd: Command,
  matches: &clap::ArgMatches,
  raw_args: Vec<String>,
  mut global_options: HashMap<String, &'static str>,
  mut global_args: Vec<&'arg clap::Arg>,
) -> napi::Result<()> {
  let mut options: HashMap<String, &'static str> = HashMap::default();
  options.extend(global_options.clone());

  for (name, option) in &cmd.options {
    let parser = leak_borrowed_str_or_default(option.parser.as_ref(), "string");
    options.entry(name.to_string()).or_insert(parser);
    if option.global.unwrap_or(false) {
      global_options.entry(name.to_string()).or_insert(parser);
    }
  }

  let mut args = clap.get_arguments().collect::<Vec<&clap::Arg>>();
  args.extend(global_args.clone());

  let global_args_this = clap
    .get_arguments()
    .filter(|arg| arg.is_global_set())
    .collect::<Vec<&clap::Arg>>();
  global_args.extend(global_args_this);

  merge_args_matches(&mut parsed_args, &args, &options, matches)?;

  if let Some((sub_command_name, sub_matches)) = matches.subcommand() {
    let mut sub_commands = cmd.subcommands.unwrap_or_default();
    let sub_command_def = sub_commands.remove(sub_command_name).unwrap();

    let sub_command = clap
      .get_subcommands()
      .find(|&sub_command| sub_command.get_name() == sub_command_name)
      .unwrap();

    parse_arguments_inner(
      env,
      parsed_args,
      sub_command,
      sub_command_def,
      sub_matches,
      raw_args,
      global_options,
      global_args,
    )?;
  } else {
    let context = Context::new(parsed_args, raw_args);
    if let Some(cb) = cmd.callback.as_ref() {
      cb.call1::<Context, JsNull>(context)?;
    } else {
      env.throw_error(
        "No callback function found for main command and no subcommand was provided.",
        Some("E_NO_CALLBACK"),
      )?;
    };
  };
  Ok(())
}

pub(crate) fn parse_arguments(
  env: Env,
  clap: &clap::Command,
  cmd: Command,
  matches: &clap::ArgMatches,
  raw_args: Vec<String>,
) -> napi::Result<()> {
  let parsed_args = env.create_object()?;

  parse_arguments_inner(
    env,
    parsed_args,
    clap,
    cmd,
    matches,
    raw_args,
    HashMap::default(),
    Vec::new(),
  )
}
