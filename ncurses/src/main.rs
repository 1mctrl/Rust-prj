use ncurses::*;
use std::io;
use std::str::FromStr;

enum OperandTypes {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct GetNum {
    first_digit: i128,
    second_digit: i128,
    operatsia: OperandTypes,
}

impl GetNum {
    fn parse_from_str(&mut self, input: &str) -> Result<(), &'static str> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Use format: <a> <op> <b>");
        }

        self.first_digit = i128::from_str(parts[0]).map_err(|_| "Invalid first number")?;
        self.second_digit = i128::from_str(parts[2]).map_err(|_| "Invalid second number")?;

        self.operatsia = match parts[1] {
            "+" => OperandTypes::Add,
            "-" => OperandTypes::Subtract,
            "*" => OperandTypes::Multiply,
            "/" => OperandTypes::Divide,
            _ => return Err("Unsupported operand"),
        };

        Ok(())
    }

    fn calculate(&self) -> String {
        match self.operatsia {
            OperandTypes::Add => format!("Result: {}", self.first_digit + self.second_digit),
            OperandTypes::Subtract => format!("Result: {}", self.first_digit - self.second_digit),
            OperandTypes::Multiply => format!("Result: {}", self.first_digit * self.second_digit),
            OperandTypes::Divide => {
                if self.second_digit == 0 {
                    "Division by zero not allowed".to_string()
                } else {
                    let quotient = self.first_digit / self.second_digit;
                    let remainder = self.first_digit % self.second_digit;
                    if remainder != 0 {
                        format!("Result: {}, Remainder: {}", quotient, remainder)
                    } else {
                        format!("Result: {}", quotient)
                    }
                }
            }
        }
    }
}

fn main() {
    // инициализация ncurses
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();

    // цветовые пары
    init_pair(1, COLOR_GREEN, -1);  // результат
    init_pair(2, COLOR_RED, -1);    // ошибки
    init_pair(3, COLOR_YELLOW, -1); // заголовок

    // окно
    let height = 10;
    let width = 50;
    let start_y = 2;
    let start_x = 4;
    let win = newwin(height, width, start_y, start_x);
    box_(win, 0, 0);

    wattron(win, COLOR_PAIR(3));
    mvwprintw(win, 0, 2, " Rust ncurses Calculator ");
    wattroff(win, COLOR_PAIR(3));
    mvwprintw(win, 2, 2, "Enter expression (e.g. 12 + -3): ");
    wrefresh(win);

    // ввод
    let mut input = String::new();
    loop {
        let ch = wgetch(win);
        if ch == 10 { // Enter
            break;
        } else if ch == 127 || ch == KEY_BACKSPACE {
            if !input.is_empty() {
                input.pop();
                mvwprintw(win, 3, 2, " ".repeat(45).as_str());
                mvwprintw(win, 3, 2, &input);
                wrefresh(win);
            }
        } else {
            let c = ch as u8 as char;
            input.push(c);
            mvwprintw(win, 3, 2, &input);
            wrefresh(win);
        }
    }

    // логика
    let mut nums = GetNum {
        first_digit: 0,
        second_digit: 0,
        operatsia: OperandTypes::Add,
    };

    match nums.parse_from_str(&input) {
        Ok(_) => {
            let result = nums.calculate();
            wattron(win, COLOR_PAIR(1));
            mvwprintw(win, 5, 2, &result);
            wattroff(win, COLOR_PAIR(1));
        }
        Err(e) => {
            wattron(win, COLOR_PAIR(2));
            mvwprintw(win, 5, 2, e);
            wattroff(win, COLOR_PAIR(2));
        }
    }

    mvwprintw(win, 7, 2, "Press any key to exit...");
    wrefresh(win);
    wgetch(win);

    endwin();
}
