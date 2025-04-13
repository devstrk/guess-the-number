use std::io::stdin;

use colored::Colorize;

use crate::constants::INPUT_ERROR_MSG;

pub fn get_input() -> String {
  let mut input: String = String::new();
  stdin()
    .read_line(&mut input)
    .expect(&INPUT_ERROR_MSG.to_string().red().bold());
  input
}

pub enum MessageColor {
  Primary,
  Success,
  Error,
  Accent,
}

pub fn print_message(message: &str, color: MessageColor) {
  match color {
    MessageColor::Primary => println!(
      "\n{}\n",
      format!(" {} ", message).on_yellow().bold().black()
    ),
    MessageColor::Success => println!("\n{}\n", format!(" {} ", message).on_green().bold().black()),
    MessageColor::Error => println!("\n{}\n", format!(" {} ", message).on_red().bold().black()),
    MessageColor::Accent => println!("{}\n", format!("{}", message).yellow().bold()),
  }
}
