use crate::{constants::*, utils::*};

#[derive(Clone)]
pub enum GameMode {
  Easy,
  Medium,
  Hard,
}

impl GameMode {
  pub fn id(&self) -> &str {
    match self {
      Self::Easy => "1",
      Self::Medium => "2",
      Self::Hard => "3",
    }
  }

  pub fn title(&self) -> &str {
    match self {
      Self::Easy => EASY_MODE_TITLE,
      Self::Medium => MEDIUM_MODE_TITLE,
      Self::Hard => HARD_MODE_TITLE,
    }
  }

  pub fn max_number(&self) -> u8 {
    match self {
      Self::Easy => 50,
      Self::Medium => 100,
      Self::Hard => 250,
    }
  }

  pub fn max_attempts(&self) -> u8 {
    match self {
      Self::Easy => EASY_MODE_MAX_ATTEMPTS,
      Self::Medium => DEFAULT_MAX_ATTEMPTS,
      Self::Hard => HARD_MODE_MAX_ATTEMPTS,
    }
  }

  pub fn choose_mode() -> Option<GameMode> {
    print_message(CHOOSE_MODE_MSG, MessageColor::Accent);

    let modes: [GameMode; 3] = [GameMode::Easy, GameMode::Medium, GameMode::Hard];

    loop {
      for mode in modes.iter() {
        println!(
          "{}. {} ({} попыток)",
          mode.id(),
          mode.title(),
          mode.max_attempts()
        );
      }
      println!("{}. Выход\n", modes.len() + 1);

      let choice: String = get_input();
      match choice.trim() {
        "1" => break Some(modes[0].clone()),
        "2" => break Some(modes[1].clone()),
        "3" => break Some(modes[2].clone()),
        "4" => break None,
        _ => {
          print_message(INVALID_MSG, MessageColor::Error);
          continue;
        }
      }
    }
  }
}
