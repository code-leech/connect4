mod misc;
use misc::switchtoken;
use misc::checkwin;
use crossterm::style::Stylize;
use crossterm::execute;
use misc::{clear, waituntil, printgrid};
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear};

use crate::misc::selectgrid;
fn main() {
    execute!(std::io::stdout(), EnterAlternateScreen, Hide).unwrap();
    let mut quit = false;
    loop {
        let mut grid = vec![String::from(" "); 42];
        let mut token = "X".red();
        loop {
            let (col, row) = selectgrid(&grid, token);
            if col == -1 && row == -1 {
                quit = true;
                break;
            }
            grid[(col + row * 7) as usize] = token.to_string();
            if checkwin(&grid, token) {
                clear();
                printgrid(&grid);
                println!("\n{} won!", token);
                waituntil();
                break;
            }
            if col == -2 && row == -2 {
                clear();
                printgrid(&grid);
                println!("\nIt's a draw!");
                waituntil();
                break;
            }
            token = switchtoken(token);
        }
        if quit == true {
            break;
        }
        execute!(std::io::stdout(), Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0)).unwrap();
    }
    execute!(std::io::stdout(), LeaveAlternateScreen, Show).unwrap();

}
