mod misc;
use misc::switchtoken;
use misc::checkwin;
use crossterm::style::Stylize;
use crossterm::execute;
use misc::{clear, waituntil, printgrid};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use crate::misc::selectgrid;
fn main() {
    execute!(std::io::stdout(), EnterAlternateScreen, Hide).unwrap();
    let mut grid = vec![String::from(" "); 42];
    let mut token = "X".red();
    loop {
        let (col, row) = selectgrid(&grid, token);
        grid[(col + row * 7) as usize] = token.to_string();
        if checkwin(&grid, token) {
            clear();
            printgrid(&grid);
            println!("\n{} won!\nPress any key to continue...", token);
            waituntil();
        }
        if col == -2 && row == -2 {
            clear();
            printgrid(&grid);
            println!("\nIt's a draw!\nPress any key to continue...");
            waituntil();
        }
        if col == -1 && row == -1 {
            break;
        }
        token = switchtoken(token);
    }
    execute!(std::io::stdout(), LeaveAlternateScreen, Show).unwrap();

}
