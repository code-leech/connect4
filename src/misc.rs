use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::{StyledContent, Stylize};
use crossterm::cursor::MoveTo;
use std::{time::Duration, thread::sleep};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ClearType};

use crossterm::execute;

fn checkdraw(grid: &Vec<String>) -> bool {
    for i in grid {
        if i == " " {
            return false;
        }
    }
    true
}

pub fn waituntil() {
    enable_raw_mode().unwrap();
    loop {
        if let Event::Key(_) = event::read().unwrap() {
            break;
        }
    }
    disable_raw_mode().unwrap();
}

pub fn checkwin(grid: &Vec<String>, token: StyledContent<&str>) -> bool {
    let rows = 6;
    let cols = 7;
    let get = |r: usize, c: usize| -> &String {
        &grid[r * cols + c]
    };
    let token_str = token.to_string();
    for r in 0..rows {
        for c in 0..cols - 3 {
            if (0..4).all(|i| get(r, c + i) == &token_str) {
                return true;
            }
        }
    }
    for r in 0..rows - 3 {
        for c in 0..cols {
            if (0..4).all(|i| get(r + i, c) == &token_str) {
                return true;
            }
        }
    }
    for r in 0..rows - 3 {
        for c in 0..cols - 3 {
            if (0..4).all(|i| get(r + i, c + i) == &token_str) {
                return true;
            }
        }
    }
    for r in 0..rows - 3 {
        for c in 3..cols {
            if (0..4).all(|i| get(r + i, c - i) == &token_str) {
                return true;
            }
        }
    }
    
    false
}

pub fn clear() {
    execute!(std::io::stdout(), crossterm::terminal::Clear(ClearType::Purge), MoveTo(0, 0)).unwrap();
}

fn checkavailable(grid: &Vec<String>, col: i16) -> bool {
    for row in 0..6 {
        if grid[(col + row * 7) as usize] == " " {
            return true;
        }
    }
    false
}

pub fn printgrid(grid: &Vec<String>) {
    execute!(
        std::io::stdout(),
        crossterm::style::Print(format!("
┌───┬───┬───┬───┬───┬───┬───┐
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
├───┼───┼───┼───┼───┼───┼───┤
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
├───┼───┼───┼───┼───┼───┼───┤
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
├───┼───┼───┼───┼───┼───┼───┤
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
├───┼───┼───┼───┼───┼───┼───┤
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
├───┼───┼───┼───┼───┼───┼───┤
│ {} │ {} │ {} │ {} │ {} │ {} │ {} │
└───┴───┴───┴───┴───┴───┴───┘", 
            grid[0], grid[1], grid[2], grid[3], grid[4], grid[5], grid[6],
            grid[7], grid[8], grid[9], grid[10], grid[11], grid[12], grid[13],
            grid[14], grid[15], grid[16], grid[17], grid[18], grid[19], grid[20],
            grid[21], grid[22], grid[23], grid[24], grid[25], grid[26], grid[27],
            grid[28], grid[29], grid[30], grid[31], grid[32], grid[33], grid[34],
            grid[35], grid[36], grid[37], grid[38], grid[39], grid[40], grid[41]
        )),
    ).unwrap();
}

pub fn switchtoken(token: StyledContent<&str>) -> StyledContent<&'static str> {
    if token == "X".red() {
        "O".blue() 
    } else {
        "X".red() 
    }
}

fn selecttoken(token: StyledContent<&str>) -> StyledContent<&'static str> {
    if token == "X".red() {
        "X".green()
    } else {
        "O".green() 
    }
}


pub fn selectgrid(grid: &Vec<String>, token: StyledContent<&str>) -> (i16, i16){
    if checkdraw(grid) {
        return(-2, -2);
    }
    let mut tempgrid: Vec<String>;
    let mut col: i16 = 0;
    let mut row: i16 = 0;
    loop {
        if checkavailable(grid, col) {
            break;
        }
        col += 1;
    }
    enable_raw_mode().unwrap();
    loop {
        clear();
        tempgrid = grid.clone();
        tempgrid[col as usize] = selecttoken(token).to_string();
        printgrid(&tempgrid);
        let mut tempcol = col; 
        
        if let Event::Key(event) = event::read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Right => {
                        if col < 6 {               
                            while tempcol < 6 {
                                tempcol += 1;
                                if checkavailable(grid, tempcol) {
                                    col = tempcol;
                                    break;
                                }
                            }
                        }
                    }
                    KeyCode::Left => {
                        if col > 0 {          
                            while tempcol > 0 {
                                tempcol -= 1;
                                if checkavailable(grid, tempcol) {
                                    col = tempcol;
                                    break;
                                }
                            }           
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Down => {
                        break;
                    }
                    KeyCode::Char('a') => {
                        if col > 0 {          
                            while tempcol > 0 {
                                tempcol -= 1;
                                if checkavailable(grid, tempcol) {
                                    col = tempcol;
                                    break;
                                }
                            }           
                        }
                    }
                    KeyCode::Esc => {
                        return(-1, -1)
                    }
                    KeyCode::Char('d') => {
                        if col < 6 {               
                            while tempcol < 6 {
                                tempcol += 1;
                                if checkavailable(grid, tempcol) {
                                    col = tempcol;
                                    break;
                                }
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        break;
                    }
                    
                    _ => {}
                }
            }
        }
    }
    for _ in 0..5 {
        if grid[(col + 7 + row * 7) as usize] == " " {
            row += 1;
            tempgrid = grid.clone();
            sleep(Duration::from_millis(75));
            tempgrid[(col + row * 7) as usize] = selecttoken(token).to_string();
            clear();
            printgrid(&tempgrid);
        }
        else {
            break
        }
    }
    disable_raw_mode().unwrap();
    (col, row)
}