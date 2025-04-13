# Угадай число 🎮

![Rust](https://img.shields.io/badge/lang-Rust-orange?logo=rust) ![CLI](https://img.shields.io/badge/interface-CLI-lightgrey)

Классическая игра "Угадай число" с поддержкой разных уровней сложности, написанная на Rust.

## 🚀 Возможности

- Три уровня сложности:
  - 🟢 Легкий (1-50, 15 попыток)
  - 🟡 Средний (1-100, 10 попыток)
  - 🔴 Сложный (1-250, 5 попыток)
- Цветной интерактивный интерфейс
- Подсчет использованных попыток
- Валидация ввода пользователя
- Возможность играть снова или изменить сложность

## 📦 Установка

1. Убедитесь, что установлен [Rust](https://www.rust-lang.org/tools/install)
2. Клонируйте репозиторий:

```bash
git clone https://github.com/devstrk/guess-the-number.git
cd guess-the-number
```

3. Соберите и запустите приложение:

```bash
cargo build --release
```

## 🖥️ Использование

1. Запустите программу:

```bash
cargo run --release
```

2. Следуйте инструкциям:

```bash
Добро пожаловать в игру 'Угадай число'!

Выберите уровень сложности:

1. Легкий (15 попыток)
2. Средний (10 попыток)
3. Сложный (5 попыток)
4. Выход

> 2

Выбран "Средний" режим

Отгадай число от 1 до 100 за 10 попыток!

> 50
Ваше число слишком маленькое!
Попробуй еще раз! (Осталось попыток: 9)

> 75
Поздравляем! Вы угадали число!
Использовано попыток: 2
```

## 🛠️ Технические детали

- Архитектура:
  - Чистое разделение логики (game, mode, utils модули)
  - Использование перечислений (enum) для режимов игры
  - Константные данные для всех сообщений
- Особенности Rust:
  - Идиоматичная обработка ошибок
  - Безопасность типов
  - Использование крейтов colored и rand

## 📝 Лицензия

<a href="LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>

См. файл <a href="LICENSE">LICENSE</a>.

---

Разработано с ❤️ на Rust | [devstrk](https://github.com/devstrk)
