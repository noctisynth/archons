use napi::{JsObject, Result};

use crate::types::Command;

pub(crate) fn leak_str(s: String) -> &'static str {
  s.leak()
}

pub(crate) fn leak_borrowed_str_or_default(s: Option<&String>, default: &str) -> &'static str {
  s.map_or_else(|| leak_str(default.to_string()), |s| leak_str(s.clone()))
}

pub(crate) fn leak_borrowed_str(s: &String) -> &'static str {
  s.to_owned().leak()
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
