// =============================================================================
// MineOS - Calculator Application (Rust)
// =============================================================================

#![allow(dead_code)]

use alloc::string::String;
use alloc::format;

/// Calculator state
pub struct Calculator {
    pub display: String,
    pub expression: String,
    pub current: f64,
    pub operator: Option<char>,
    pub new_number: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            display: String::from("0"),
            expression: String::new(),
            current: 0.0,
            operator: None,
            new_number: true,
        }
    }

    /// Handle number input
    pub fn input_number(&mut self, digit: char) {
        if self.new_number {
            self.display = String::from(digit);
            self.new_number = false;
        } else {
            if self.display == "0" && digit != '.' {
                self.display = String::from(digit);
            } else {
                self.display.push(digit);
            }
        }
    }

    /// Handle operator input
    pub fn input_operator(&mut self, op: char) {
        let value: f64 = self.display.parse().unwrap_or(0.0);
        
        if let Some(prev_op) = self.operator {
            self.current = self.calculate(self.current, value, prev_op);
            self.display = format_number(self.current);
        } else {
            self.current = value;
        }
        
        self.operator = Some(op);
        self.expression = format!("{} {} ", format_number(self.current), op);
        self.new_number = true;
    }

    /// Calculate equals
    pub fn equals(&mut self) {
        let value: f64 = self.display.parse().unwrap_or(0.0);
        
        if let Some(op) = self.operator {
            self.current = self.calculate(self.current, value, op);
            self.display = format_number(self.current);
            self.expression.clear();
            self.operator = None;
            self.new_number = true;
        }
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.display = String::from("0");
        self.expression.clear();
        self.current = 0.0;
        self.operator = None;
        self.new_number = true;
    }

    /// Backspace
    pub fn backspace(&mut self) {
        if self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = String::from("0");
            self.new_number = true;
        }
    }

    /// Toggle sign
    pub fn toggle_sign(&mut self) {
        if self.display.starts_with('-') {
            self.display.remove(0);
        } else if self.display != "0" {
            self.display.insert(0, '-');
        }
    }

    /// Percentage
    pub fn percent(&mut self) {
        let value: f64 = self.display.parse().unwrap_or(0.0);
        let result = value / 100.0;
        self.display = format_number(result);
        self.new_number = true;
    }

    /// Perform a calculation
    fn calculate(&self, a: f64, b: f64, op: char) -> f64 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => if b != 0.0 { a / b } else { f64::NAN },
            _ => b,
        }
    }
}

/// Format a number for display
fn format_number(n: f64) -> String {
    if n != n {
        // NaN check without method call
        return String::from("Error");
    }
    // Check if integer by comparing to truncated value
    let truncated = n as i64 as f64;
    if n == truncated && n > -1e15 && n < 1e15 {
        format!("{}", n as i64)
    } else {
        let s = format!("{:.8}", n);
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        String::from(s)
    }
}
