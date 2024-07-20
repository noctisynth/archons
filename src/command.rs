use napi::{bindgen_prelude::*, JsNull};
use napi_derive::napi;

use crate::types::{Command, Context};
use crate::utils::{merge_args_matches, resolve_command, resolve_option_args};

#[napi]
pub fn define_command(options: Command) -> Command {
  options
}

#[napi]
pub fn run(env: Env, cmd: Command, args: Option<Vec<String>>) -> Result<()> {
  let args = resolve_option_args(args);
  let clap = resolve_command(clap::Command::default(), Default::default(), &cmd)
    .color(clap::ColorChoice::Always);
  let matches = clap.clone().get_matches_from(&args);

  let mut parsed_args = env.create_object()?;

  merge_args_matches(&mut parsed_args, &cmd, &matches)?;

  if let Some((sub_command, sub_matches)) = matches.subcommand() {
    let sub_commands = &cmd.subcommands.unwrap_or_default();
    let sub_command = sub_commands.get(sub_command).unwrap();
    let cb = sub_command.callback.as_ref().unwrap();
    merge_args_matches(&mut parsed_args, sub_command, sub_matches)?;
    let context = Context {
      args: parsed_args,
      raw_args: args,
    };
    cb.call1::<Context, JsNull>(context)?;
  } else {
    let context = Context {
      args: parsed_args,
      raw_args: args,
    };
    if let Some(cb) = cmd.callback.as_ref() {
      cb.call1::<Context, JsNull>(context)?;
    } else {
      env.throw_error("No callback function found for command", None)?;
    };
  }
  Ok(())
}
