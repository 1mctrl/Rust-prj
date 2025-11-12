use std::io;

enum OperandTypes {
    Add,
    Substract,
    Multiply,
    Divide,
}


struct GetNum {
    first_digit: i128,
    second_digit: i128,
    operatsia: OperandTypes
}

impl GetNum {
    fn input (&mut self) {
        let mut inpt = String::new();
        io::stdin().read_line(&mut inpt).unwrap();

        let parts: Vec<&str> = inpt.split_whitespace().collect();
        if parts.len() != 3 {
            println!("<digit> <operand> <digit>");
        }
        self.first_digit = parts[0].parse::<i128>().expect("first is not a digit tho");
        self.second_digit = parts[2].parse::<i128>().expect("second is not a digit tho");

        self.operatsia = match parts[1] {
            "+" => OperandTypes::Add,
            "-" => OperandTypes::Substract,
            "*" => OperandTypes::Multiply,
            "/" => OperandTypes::Divide,
            _ => panic!("cant recognize operand"),
        }
                
    }


fn calculate (&self) {
    match self.operatsia {
        OperandTypes::Add => {
            println!("Result: {}", self.first_digit + self.second_digit);
        }
        OperandTypes::Substract => {
            println!("Result: {}", self.first_digit - self.second_digit);
        }
        OperandTypes::Multiply => {
            println!("Result: {}", self.first_digit * self.second_digit);
        }
        OperandTypes::Divide => {
            if self.second_digit != 0 {
                println!("Result: {}", self.first_digit / self.second_digit);
                if self.first_digit % self.second_digit != 0 {
                    panic!("Reminder: {}", self.first_digit % self.second_digit);
                }
            }
            else {
                panic!("Division by 0 is not allowed");
            }
        }
    }
}
}

fn main () {
    loop {
    let mut nums = GetNum {
        first_digit: 0,
        second_digit: 0,
        operatsia: OperandTypes::Add
    };
    
    println!("Math");
    nums.input();
    nums.calculate();
    }
}
