// =============================================================================
// MineOS - Terminal Application (Rust)
// =============================================================================
// A command-line terminal that runs inside the MineOS kernel.
// Supports basic commands for system interaction.
// =============================================================================

use crate::drivers::{vga_text, keyboard};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

/// Built-in terminal commands
const HELP_TEXT: &str = 
"MineOS Terminal v1.0.0 - Available Commands:
  help      Show this help message
  clear     Clear the terminal screen
  echo      Echo text back
  whoami    Show current user
  uname     Show system information  
  ls        List files
  pwd       Print working directory
  date      Show uptime
  neofetch  System information display
  calc      Calculate expression (e.g. calc 2+3)
  reboot    Restart the system
  shutdown  Power off";

/// Run a text-mode shell (fallback when no framebuffer)
pub fn run_text_mode_shell() -> ! {
    vga_text::clear_screen();
    vga_text::set_color(0x0B); // Cyan
    vga_text::print_str("MineOS Terminal v1.0.0\n");
    vga_text::print_str("Type 'help' for available commands.\n\n");
    
    loop {
        // Print prompt
        vga_text::set_color(0x0B); // Cyan
        vga_text::print_str("user@mineos:~$ ");
        vga_text::set_color(0x0F); // White
        
        // Read command
        let line = keyboard::read_line(Some(echo_char));
        
        // Process command
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            execute_command(trimmed);
        }
    }
}

/// Echo a character to VGA text output
fn echo_char(ch: char) {
    if ch == '\x08' {
        // Backspace: move cursor back, print space, move back again
        vga_text::print_str("\x08 \x08");
    } else {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        vga_text::print_str(s);
    }
}

/// Execute a terminal command
pub fn execute_command(cmd: &str) {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let command = parts[0];
    let args = if parts.len() > 1 { parts[1] } else { "" };
    
    match command {
        "help" => {
            vga_text::set_color(0x0F);
            vga_text::print_str(HELP_TEXT);
            vga_text::print_str("\n");
        }
        "clear" => {
            vga_text::clear_screen();
        }
        "echo" => {
            vga_text::set_color(0x07);
            vga_text::print_str(args);
            vga_text::print_str("\n");
        }
        "whoami" => {
            vga_text::set_color(0x07);
            vga_text::print_str("user@mineos\n");
        }
        "uname" => {
            vga_text::set_color(0x07);
            vga_text::print_str("MineOS 1.0.0 x86_64 MineKernel\n");
        }
        "pwd" => {
            vga_text::set_color(0x07);
            vga_text::print_str("/home/user\n");
        }
        "ls" => {
            vga_text::set_color(0x0B); // Cyan for folders
            vga_text::print_str("Documents  ");
            vga_text::print_str("Pictures  ");
            vga_text::print_str("Music  ");
            vga_text::print_str("Downloads\n");
            vga_text::set_color(0x07); // White for files
            vga_text::print_str("readme.txt  notes.txt\n");
        }
        "date" => {
            vga_text::set_color(0x07);
            vga_text::print_str("System uptime: running since boot\n");
        }
        "neofetch" => {
            vga_text::set_color(0x0B);
            vga_text::print_str("       ___  ___  _                 ___  _____ \n");
            vga_text::print_str("       |  \\/  | (_)               /   |/  ___|\n");
            vga_text::print_str("       | .  . |  _  _ __    ___  / /| |\\ `--. \n");
            vga_text::print_str("       | |\\/| | | || '_ \\  / _ \\/ /_| | `--. \\\n");
            vga_text::print_str("       | |  | | | || | | ||  __/\\___  |/\\__/ /\n");
            vga_text::print_str("       \\_|  |_/ |_||_| |_| \\___|    |_/\\____/ \n");
            vga_text::set_color(0x07);
            vga_text::print_str("\n  OS: MineOS 1.0.0\n");
            vga_text::print_str("  Kernel: MineKernel 1.0\n");
            vga_text::print_str("  Shell: mine-sh 1.0\n");
            vga_text::print_str("  Architecture: x86_64\n");
            vga_text::print_str("  Terminal: MineOS Terminal\n\n");
        }
        "calc" => {
            if args.is_empty() {
                vga_text::set_color(0x04);
                vga_text::print_str("Usage: calc <expression>\n");
            } else {
                vga_text::set_color(0x07);
                vga_text::print_str("Calculator: ");
                vga_text::print_str(args);
                vga_text::print_str(" = ");
                // Simple integer calculation
                if let Some(result) = simple_calc(args) {
                    let result_str = format!("{}", result);
                    vga_text::print_str(&result_str);
                } else {
                    vga_text::print_str("Error");
                }
                vga_text::print_str("\n");
            }
        }
        "reboot" => {
            vga_text::set_color(0x0E);
            vga_text::print_str("Rebooting...\n");
            // Triple fault to reboot
            unsafe {
                let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
                port.write(0xFE);
            }
        }
        "shutdown" => {
            vga_text::set_color(0x0E);
            vga_text::print_str("Shutting down MineOS...\n");
            vga_text::print_str("It is now safe to turn off your computer.\n");
            loop { x86_64::instructions::hlt(); }
        }
        _ => {
            vga_text::set_color(0x04); // Red
            vga_text::print_str("mine-sh: ");
            vga_text::print_str(command);
            vga_text::print_str(": command not found\n");
        }
    }
}

/// Simple integer calculator
fn simple_calc(expr: &str) -> Option<i64> {
    // Very basic: supports single operation (a op b)
    let expr = expr.trim();
    
    for op in ['+', '-', '*', '/'] {
        if let Some(pos) = expr[1..].find(op) {
            let pos = pos + 1;
            let left: i64 = expr[..pos].trim().parse().ok()?;
            let right: i64 = expr[pos+1..].trim().parse().ok()?;
            return match op {
                '+' => Some(left + right),
                '-' => Some(left - right),
                '*' => Some(left * right),
                '/' => if right != 0 { Some(left / right) } else { None },
                _ => None,
            };
        }
    }
    
    // Try parsing as a single number
    expr.parse().ok()
}
