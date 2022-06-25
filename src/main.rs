extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("A simple wrapper for tmux.")
        .version("0.0.1")
        .author("Chad Baker <calbaker@gmail.com>")
        // .about("Teaches argument parsing")
        .arg(Arg::with_name("new")
                 .short('n')
                 .long("new")
                 .takes_value(true)
                 .help("Create new session with provided name."))
        .arg(Arg::with_name("attach")
                 .short('a')
                 .long("attach")
                 .takes_value(true)
                 .help("Attached to session with provided name."))
        .get_matches();

        println!("new: {}", matches.value_of("new").unwrap_or("not provided"));
        println!("attach: {}", matches.value_of("attach").unwrap_or("not provided"));
}
