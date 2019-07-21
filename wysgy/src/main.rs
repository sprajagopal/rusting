#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
mod cli;
mod curses;
mod project;

fn main() {
    curses::curses();
}
