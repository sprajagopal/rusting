#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
mod callbacks;
mod cli;
mod curses;
mod info;
mod layouts;
mod panes;
mod project;

fn main() {
    cli::cli();
}
