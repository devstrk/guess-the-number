use std::cmp::Ordering;

use colored::Colorize;
use rand::random_range;

use crate::{
  constants::{
    CALL_TO_ACTION_MSG, GAME_AGAIN_MSG, GAMEOVER_MSG, GAMEWIN_MSG, GUESS_AGAIN_MSG, INVALID_MSG,
    MIN_NUMBER, TO_HIGH_MSG, TO_LOW_MSG,
  },
  mode::GameMode,
  utils::{MessageColor, get_input, print_message},
};

pub fn run_game_loop(mode: GameMode) {
  loop {
    let secret_number: u8 = random_range(MIN_NUMBER..=mode.max_number());
    let mut attempts: u8 = 0;
    let invalid_msg = format!("{} {}!", INVALID_MSG, mode.max_number());
    let call_to_action_msg = format!(
      "{} {} за {} {}",
      CALL_TO_ACTION_MSG,
      mode.max_number(),
      mode.max_attempts().to_string(),
      "попыток!"
    );
    print_message(&call_to_action_msg, MessageColor::Accent);

    loop {
      if attempts == mode.max_attempts() {
        print_message(GAMEOVER_MSG, MessageColor::Error);
        break;
      }

      let input: u8 = match get_input().trim().parse::<u8>() {
        Ok(input) if (MIN_NUMBER..=mode.max_number()).contains(&input) => input,
        Ok(_) => {
          print_message(&invalid_msg, MessageColor::Error);
          continue;
        }
        Err(_) => {
          print_message(&invalid_msg, MessageColor::Error);
          continue;
        }
      };

      attempts += 1;

      if compare_guess(&mode, attempts, input, secret_number) {
        break;
      }
    }

    match get_replay_choice(&mode, &invalid_msg) {
      ReplayChoice::Continue => {
        println!();
        continue;
      }
      ReplayChoice::ChangeMode => break,
    }
  }
}

fn compare_guess(mode: &GameMode, counter: u8, input: u8, secret_number: u8) -> bool {
  let attempts = mode.max_attempts() - counter;

  match input.cmp(&secret_number) {
    Ordering::Less => wrong_guess(attempts, TO_LOW_MSG),
    Ordering::Greater => wrong_guess(attempts, TO_HIGH_MSG),
    Ordering::Equal => write_guess(counter),
  }
}

fn wrong_guess(attempts: u8, message: &str) -> bool {
  println!("\n{}", message.bold().red());
  if attempts > 0 {
    println!(
      "\n{} (Осталось попыток: {})\n",
      GUESS_AGAIN_MSG.bold().yellow(),
      attempts
    );
  }

  false
}

fn write_guess(attempts: u8) -> bool {
  print_message(GAMEWIN_MSG, MessageColor::Success);
  println!(
    "{}",
    format!("Использовано попыток: {}", attempts.to_string()).bright_black()
  );

  true
}

enum ReplayChoice {
  Continue,
  ChangeMode,
}

fn get_replay_choice(mode: &GameMode, invalid_msg: &str) -> ReplayChoice {
  loop {
    print_message(GAME_AGAIN_MSG, MessageColor::Primary);
    println!(
      "1. Да (режим: {}, {} попыток)",
      mode.title(),
      mode.max_attempts()
    );
    println!("2. Изменить сложность\n");

    match get_input().trim() {
      "1" => return ReplayChoice::Continue,
      "2" => return ReplayChoice::ChangeMode,
      _ => print_message(invalid_msg, MessageColor::Error),
    }
  }
}
