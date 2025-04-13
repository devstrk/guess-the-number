mod constants;
mod game;
mod mode;
mod utils;

use constants::{GAME_GOODBYE_MSG, GAME_INTRO_MSG};
use game::run_game_loop;
use mode::GameMode;
use utils::{MessageColor, print_message};

fn main() {
  loop {
    print_message(GAME_INTRO_MSG, MessageColor::Primary);

    let mode: GameMode = match GameMode::choose_mode() {
      Some(mode) => mode,
      None => break,
    };

    print_message(
      &format!("{}{}{}", "Выбран \"", mode.title(), "\" режим"),
      MessageColor::Primary,
    );

    run_game_loop(mode);
  }

  print_message(GAME_GOODBYE_MSG, MessageColor::Success);
}
