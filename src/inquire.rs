use std::rc::Rc;

use inquire::validator::{ErrorMessage, StringValidator, Validation};
use napi::JsFunction;
use napi_derive::napi;

use crate::{
  apply_opt,
  types::{Context, Error},
  utils::{as_usize, leak_str, wrap_bool_formatter, wrap_bool_parser, wrap_string_formatter},
};

#[napi(object)]
#[derive(Default)]
pub struct SelectConfig {
  pub help_message: Option<String>,
  pub page_size: Option<u32>,
  pub reset_cursor: Option<bool>,
  pub starting_cursor: Option<u32>,
  pub starting_filter_input: Option<String>,
  pub vim_mode: Option<bool>,
  pub filtering: Option<bool>,
  pub help_message_disabled: Option<bool>,
}

#[napi]
pub fn select(
  prompt: String,
  choices: Vec<String>,
  config: Option<SelectConfig>,
) -> napi::Result<String> {
  let mut inquire = inquire::Select::new(&prompt, choices);
  let config = config.unwrap_or_default();

  apply_opt!(inquire, config, leak_str(help_message) => with_help_message);
  apply_opt!(inquire, config, as_usize(page_size) => with_page_size);
  apply_opt!(inquire, config, reset_cursor => with_reset_cursor);
  apply_opt!(inquire, config, as_usize(starting_cursor) => with_starting_cursor);
  apply_opt!(inquire, config, leak_str(starting_filter_input) => with_starting_filter_input);
  apply_opt!(inquire, config, vim_mode => with_vim_mode);

  if let Some(false) = config.filtering {
    inquire = inquire.without_filtering();
  }
  if let Some(true) = config.help_message_disabled {
    inquire = inquire.without_help_message();
  }

  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct CheckboxConfig {
  pub all_selected_by_default: Option<bool>,
  pub default: Option<Vec<u32>>,
  pub help_message: Option<String>,
  pub keep_filter: Option<bool>,
  pub page_size: Option<u32>,
  pub reset_cursor: Option<bool>,
  pub starting_cursor: Option<u32>,
  pub starting_filter_input: Option<String>,
  pub vim_mode: Option<bool>,
  pub filtering: Option<bool>,
  pub help_message_disabled: Option<bool>,
}

#[napi]
pub fn checkbox(
  prompt: String,
  choices: Vec<String>,
  config: Option<CheckboxConfig>,
) -> napi::Result<Vec<String>> {
  let mut inquire = inquire::MultiSelect::new(&prompt, choices);
  let config = config.unwrap_or_default();
  if let Some(true) = config.all_selected_by_default {
    inquire = inquire.with_all_selected_by_default();
  }
  if let Some(default) = config.default {
    let default = Box::new(
      default
        .into_iter()
        .map(|i| i as usize)
        .collect::<Vec<usize>>(),
    )
    .leak();
    inquire = inquire.with_default(default);
  }
  // TODO: formatter
  // if let Some(formatter) = config.formatter {
  // }
  apply_opt!(inquire, config, leak_str(help_message) => with_help_message);
  apply_opt!(inquire, config, keep_filter => with_keep_filter);
  apply_opt!(inquire, config, as_usize(page_size) => with_page_size);
  apply_opt!(inquire, config, reset_cursor => with_reset_cursor);
  // TODO: scorer
  // if let Some(scorer) = config.scorer {
  // }
  apply_opt!(inquire, config, as_usize(starting_cursor) => with_starting_cursor);
  apply_opt!(inquire, config, leak_str(starting_filter_input) => with_starting_filter_input);
  // TODO: validator
  // if let Some(validator) = config.validator {
  // }
  apply_opt!(inquire, config, vim_mode => with_vim_mode);

  if let Some(false) = config.filtering {
    inquire = inquire.without_filtering();
  }
  if let Some(true) = config.help_message_disabled {
    inquire = inquire.without_help_message();
  }

  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct InputConfig {
  pub default: Option<String>,
  #[napi(ts_type = "(value: string) => string")]
  pub formatter: Option<JsFunction>,
  pub help_message: Option<String>,
  pub initial_value: Option<String>,
  pub page_size: Option<u32>,
  pub placeholder: Option<String>,
  #[napi(ts_type = "((text: string) => StringValidatorResult)[]")]
  pub validators: Option<Vec<JsFunction>>,
}

#[napi]
pub fn input(prompt: String, config: Option<InputConfig>) -> napi::Result<String> {
  let mut inquire = inquire::Text::new(&prompt);
  let config = config.unwrap_or_default();
  apply_opt!(inquire, config, leak_str(default) => with_default);
  apply_opt!(inquire, config, wrap_string_formatter(formatter) => with_formatter);
  apply_opt!(inquire, config, leak_str(help_message) => with_help_message);
  apply_opt!(inquire, config, leak_str(initial_value) => with_initial_value);
  apply_opt!(inquire, config, as_usize(page_size) => with_page_size);
  apply_opt!(inquire, config, leak_str(placeholder) => with_placeholder);
  if let Some(validators) = config.validators {
    let validators: Vec<Box<dyn StringValidator>> = validators
      .into_iter()
      .map(|f| validator(Rc::new(f)))
      .collect();
    inquire = inquire.with_validators(&validators);
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct ConfirmConfig {
  pub default: Option<bool>,
  #[napi(ts_type = "(value: boolean) => string")]
  pub default_value_formatter: Option<JsFunction>,
  pub error_message: Option<String>,
  #[napi(ts_type = "(value: boolean) => string")]
  pub formatter: Option<JsFunction>,
  pub help_message: Option<String>,
  #[napi(ts_type = "(value: boolean) => boolean")]
  pub parser: Option<JsFunction>,
  pub placeholder: Option<String>,
  pub starting_input: Option<String>,
}

#[napi]
pub fn confirm(prompt: String, config: Option<ConfirmConfig>) -> napi::Result<bool> {
  let mut inquire = inquire::Confirm::new(&prompt);
  let config = config.unwrap_or_default();
  apply_opt!(inquire, config, default => with_default);
  apply_opt!(inquire, config, wrap_bool_formatter(default_value_formatter) => with_default_value_formatter);
  apply_opt!(inquire, config, leak_str(error_message) => with_error_message);
  apply_opt!(inquire, config, wrap_bool_formatter(formatter) => with_formatter);
  apply_opt!(inquire, config, leak_str(help_message) => with_help_message);
  apply_opt!(inquire, config, wrap_bool_parser(parser) => with_parser);
  apply_opt!(inquire, config, leak_str(placeholder) => with_placeholder);
  apply_opt!(inquire, config, leak_str(starting_input) => with_starting_input);
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct PasswordConfig {
  pub custom_confirmation_error_message: Option<String>,
  pub custom_confirmation_message: Option<String>,
  #[napi(ts_type = "'hidden' | 'masked' | 'full'")]
  pub display_mode: Option<String>,
  pub display_toggle: Option<bool>,
  pub help_message: Option<String>,
  #[napi(ts_type = "(text: string) => string")]
  pub formatter: Option<JsFunction>,
  #[napi(ts_type = "((text: string) => StringValidatorResult)[]")]
  pub validators: Option<Vec<JsFunction>>,
  pub confirmation: Option<bool>,
}

#[napi(object)]
pub struct StringValidatorResult {
  #[napi(ts_type = "'valid' | 'invalid'")]
  pub validation: String,
  pub err_msg: Option<String>,
}

fn validator(f: Rc<JsFunction>) -> Box<dyn StringValidator> {
  Box::new(move |text: &str| {
    let res: StringValidatorResult = f.call1(text)?;
    Ok(match res.validation.as_str() {
      "valid" => Validation::Valid,
      "invalid" => Validation::Invalid(ErrorMessage::Custom(
        res.err_msg.unwrap_or("invalid".to_string()),
      )),
      _ => panic!("invalid validation result"),
    })
  })
}

#[napi]
pub fn password(prompt: String, config: Option<PasswordConfig>) -> napi::Result<String> {
  let mut inquire = inquire::Password::new(&prompt);
  let config = config.unwrap_or_default();
  apply_opt!(inquire, config, leak_str(custom_confirmation_error_message) => with_custom_confirmation_error_message);
  apply_opt!(inquire, config, leak_str(custom_confirmation_message) => with_custom_confirmation_message);
  if let Some(display_mode) = config.display_mode {
    inquire = inquire.with_display_mode(match display_mode.as_str() {
      "hidden" => inquire::PasswordDisplayMode::Full,
      "masked" => inquire::PasswordDisplayMode::Masked,
      "full" => inquire::PasswordDisplayMode::Hidden,
      _ => {
        return Err(
          Error::InquireError(inquire::InquireError::InvalidConfiguration(
            "invalid password display mode".to_string(),
          ))
          .into(),
        );
      }
    });
  }
  if let Some(true) = config.display_toggle {
    inquire = inquire.with_display_toggle_enabled();
  }
  apply_opt!(inquire, config, leak_str(help_message) => with_help_message);
  apply_opt!(inquire, config, wrap_string_formatter(formatter) => with_formatter);
  if let Some(false) = config.confirmation {
    inquire = inquire.without_confirmation();
  }
  if let Some(validators) = config.validators {
    let validators: Vec<Box<dyn StringValidator>> = validators
      .into_iter()
      .map(|f| validator(Rc::new(f)))
      .collect();
    inquire = inquire.with_validators(&validators);
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi]
impl Context {
  #[napi]
  pub fn ask(&self, prompt: String, config: Option<InputConfig>) -> napi::Result<String> {
    input(prompt, config)
  }

  #[napi]
  pub fn confirm(&self, prompt: String, config: Option<ConfirmConfig>) -> napi::Result<bool> {
    confirm(prompt, config)
  }
}
