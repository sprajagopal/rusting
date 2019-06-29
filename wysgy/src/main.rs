#[macro_use]
extern crate clap;
mod cli;
mod curses;
mod project;

fn main() {
    curses::curses();
}
