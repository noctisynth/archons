use std::time::Duration;

use napi_derive::napi;

use crate::types::{Context, Error};

#[napi]
pub struct ProgressBar {
  bar: indicatif::ProgressBar,
}

#[napi]
impl ProgressBar {
  #[napi]
  pub fn finish(&self) {
    self.bar.finish();
  }

  #[napi]
  pub fn finish_and_clear(&self) {
    self.bar.finish_and_clear();
  }

  #[napi]
  pub fn finish_using_style(&self) {
    self.bar.finish_using_style();
  }

  #[napi]
  pub fn finish_with_message(&self, msg: String) {
    self.bar.finish_with_message(msg);
  }

  #[napi]
  pub fn set_position(&self, pos: u32) {
    self.bar.set_position(pos as u64);
  }

  #[napi]
  pub fn set_length(&self, len: u32) {
    self.bar.set_length(len as u64);
  }

  #[napi]
  pub fn set_message(&self, msg: String) {
    self.bar.set_message(msg);
  }

  #[napi]
  pub fn set_prefix(&self, prefix: String) {
    self.bar.set_prefix(prefix);
  }

  #[napi]
  pub fn set_tab_width(&mut self, width: u32) {
    self.bar.set_tab_width(width as usize);
  }

  #[napi]
  pub fn set_template(&mut self, template: String) -> napi::Result<()> {
    self.bar.set_style(
      self
        .bar
        .style()
        .template(&template)
        .map_err(Error::IndicatifTemplateError)?,
    );
    Ok(())
  }

  #[napi]
  pub fn set_tick_strings(&self, s: Vec<String>) {
    self.bar.set_style(
      self
        .bar
        .style()
        .tick_strings(&s.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
    );
  }

  #[napi]
  pub fn set_progress_chars(&self, s: String) {
    self
      .bar
      .set_style(self.bar.style().progress_chars(s.as_str()));
  }

  #[napi]
  pub fn tick(&self) {
    self.bar.tick();
  }

  #[napi]
  pub fn abandon(&self) {
    self.bar.abandon();
  }

  #[napi]
  pub fn abandon_with_message(&self, msg: String) {
    self.bar.abandon_with_message(msg);
  }

  #[napi]
  pub fn inc(&self, delta: u32) {
    self.bar.inc(delta as u64);
  }

  #[napi]
  pub fn inc_length(&self, delta: u32) {
    self.bar.inc_length(delta as u64);
  }

  #[napi]
  pub fn reset(&self) {
    self.bar.reset();
  }

  #[napi]
  pub fn println(&self, msg: String) {
    self.bar.println(msg);
  }

  #[napi]
  pub fn suspend<F: Fn() -> napi::Result<()>>(&self, f: F) -> napi::Result<()> {
    self.bar.suspend(f)
  }

  #[napi]
  pub fn enable_steady_tick(&self, ms: u32) {
    self
      .bar
      .enable_steady_tick(Duration::from_millis(ms as u64));
  }

  #[napi]
  pub fn disable_steady_tick(&self) {
    self.bar.disable_steady_tick();
  }
}

#[napi]
impl Context {
  #[napi]
  pub fn create_progress_bar(&self, total: u32) -> ProgressBar {
    let bar = indicatif::ProgressBar::new(total as u64);
    ProgressBar { bar }
  }

  #[napi]
  pub fn create_spinner(&self) -> ProgressBar {
    let bar = indicatif::ProgressBar::new_spinner();
    ProgressBar { bar }
  }
}

/// Creates a new progress bar with the specified total number of steps.
///
/// # Arguments
///
/// * `total` - The total number of steps for the progress bar.
///
/// # Returns
///
/// A new `ProgressBar` instance.
#[napi]
pub fn create_progress_bar(total: u32) -> ProgressBar {
  let bar = indicatif::ProgressBar::new(total as u64);
  ProgressBar { bar }
}

/// Creates a new spinner progress bar.
///
/// # Returns
///
/// A new `ProgressBar` instance with a spinner style.
#[napi]
pub fn create_spinner() -> ProgressBar {
  let bar = indicatif::ProgressBar::new_spinner();
  ProgressBar { bar }
}
