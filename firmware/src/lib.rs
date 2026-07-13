#![no_std]
use core::fmt::{Display, Write};
use core::{fmt, iter::StepBy};
use heapless::{String, Vec, format};
use rtt_target::rprintln;

#[derive(Copy, Clone, Debug)]
pub enum Symbol {
    Number(i32),
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Number(number) => write!(f, "{}", number),
            Symbol::Addition => write!(f, "+"),
            Symbol::Subtraction => write!(f, "-"),
            Symbol::Multiplication => write!(f, "*"),
            Symbol::Division => write!(f, "/"),
        }
    }
}

pub enum Action {
    Insert(Symbol),
    Calculate,
    ///Deletes last symbol
    Delete,
    AllClear,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Insert(symbol) => write!(f, "{}", symbol),
            Action::Calculate => write!(f, "="),
            Action::Delete => write!(f, "<"),
            Action::AllClear => write!(f, "AC"),
        }
    }
}
pub struct State {
    last_result: Option<i32>,
    calculation: Vec<Symbol, 3>,
}

impl State {
    pub fn init() -> State {
        State {
            // number1, operation, number2
            calculation: Vec::new(),
            last_result: None,
        }
    }

    pub fn get_last_result(&self) -> Option<i32> {
        self.last_result
    }

    pub fn get_calculation(&self) -> Vec<Symbol, 3> {
        self.calculation.clone()
    }

    pub fn get_calculation_as_string(&self) -> String<32> {
        let mut buf: String<32> = String::new();
        match self.calculation.len() {
            1 => write!(buf, "{}", self.calculation[0]).unwrap(),
            2 => write!(buf, "{} {}", self.calculation[0], self.calculation[1]).unwrap(),
            3 => write!(
                buf,
                "{} {} {}",
                self.calculation[0], self.calculation[1], self.calculation[2]
            )
            .unwrap(),
            _ => (),
        }
        buf
    }

    pub fn action(&mut self, action: Action) {
        match action {
            Action::Insert(symbol) => {
                let mut symbol = symbol;
                if let Symbol::Number(number) = symbol
                    && let Some(Symbol::Number(prev)) = self.calculation.last()
                {
                    let new_number = prev * 10 + number;
                    self.calculation.pop().unwrap();
                    symbol = Symbol::Number(new_number);
                }
                match self.calculation.push(symbol) {
                    Ok(_) => {}
                    Err(_) => rprintln!("Error: Attempted to add another symbol"),
                }
            }
            Action::Calculate => {
                if self.calculation.len() == 0 {
                    return;
                }
                if let Symbol::Number(number1) = self.calculation[0]
                    && let Symbol::Number(number2) = self.calculation[2]
                {
                    match self.calculation[1] {
                        Symbol::Number(_) => {
                            rprintln!("Error: no operator")
                        }
                        Symbol::Addition => {
                            let result = number1 + number2;
                            self.calculation.clear();
                            self.last_result = Some(result);
                        }
                        Symbol::Subtraction => {
                            let result = number1 + number2;
                            self.calculation.clear();
                            self.last_result = Some(result);
                        }
                        Symbol::Multiplication => {
                            let result = number1 + number2;
                            self.calculation.clear();
                            self.last_result = Some(result);
                        }
                        Symbol::Division => {
                            let result = number1 / number2;
                            self.calculation.clear();
                            self.last_result = Some(result);
                        }
                    }
                }
            }
            Action::Delete => {
                self.calculation.pop().unwrap();
            }

            Action::AllClear => {
                self.calculation.clear();
                self.last_result = None;
            }
        }
    }
}
