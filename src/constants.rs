use const_format::formatcp;

pub const EASY_MODE_MAX_ATTEMPTS: u8 = 15;
pub const DEFAULT_MAX_ATTEMPTS: u8 = 10;
pub const HARD_MODE_MAX_ATTEMPTS: u8 = 5;

pub const MIN_NUMBER: u8 = 1;

pub const GAME_INTRO_MSG: &str = "Добро пожаловать в игру 'Угадай число'!";
pub const CHOOSE_MODE_MSG: &str = "Выберите уровень сложности:";

pub const EASY_MODE_TITLE: &str = "Легкий";
pub const MEDIUM_MODE_TITLE: &str = "Средний";
pub const HARD_MODE_TITLE: &str = "Сложный";

pub const CALL_TO_ACTION_MSG: &str = formatcp!("Отгадай число от {} до", MIN_NUMBER);

pub const INVALID_MSG: &str = formatcp!("Пожалуйста, введите число от {} до", MIN_NUMBER);
pub const INPUT_ERROR_MSG: &str = "\nОшибка ввода, повторите попытку.";

pub const TO_LOW_MSG: &str = "Ваше число слишком маленькое!";
pub const TO_HIGH_MSG: &str = "Ваше число слишком большое!";
pub const GUESS_AGAIN_MSG: &str = "Попробуй еще раз!";

pub const GAMEOVER_MSG: &str = "Игра окончена! Вы исчерпали максимальное кол-во попыток.";
pub const GAMEWIN_MSG: &str = "Поздравляем! Вы угадали число!";
pub const GAME_AGAIN_MSG: &str = "Сыграем еще?";

pub const GAME_GOODBYE_MSG: &str = "До свидания!";
