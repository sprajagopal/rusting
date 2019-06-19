extern crate ncurses;

use std::char;
use ncurses::*;

fn main() {
    initscr();
    raw();

    printw("Enter a character");
    let ch = getch();
    printw("You entered");
    printw(&ch.to_string()); 
    refresh();
    let ch = getch();
    endwin(); 
}
