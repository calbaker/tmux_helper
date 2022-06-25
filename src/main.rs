use clap::{Arg, App};
use std::process::Command;

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
                 .help("Attach to session with provided name."))
        .get_matches();

    match matches.value_of("new") {
        Some(session_name) => {
            Command::new("tmux").args(&["new", "-s", session_name] ).status().unwrap();
            assert!(matches.value_of("attach").is_none())
        }
        None => {
            match matches.value_of("attach") {
                Some(session_name) => {
                    Command::new("tmux").args(&["a", "-t", session_name] ).status().unwrap();
                }
                None => {
                    panic!("Must provide either `-a` or `-n` followed by session name.")
                }
            }
        }
    }
}
