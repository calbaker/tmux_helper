use clap::{Arg, Command};
use std::process::Command as ProcCommand;

fn main() {
    let matches = Command::new("A simple wrapper for tmux.")
        .version("0.0.1")
        .author("Chad Baker <calbaker@gmail.com>")
        // .about("Teaches argument parsing")
        .arg(
            Arg::with_name("new")
                .short('n')
                .long("new")
                .takes_value(true)
                .exclusive(true)
                .help("Create new session with provided name."),
        )
        .arg(
            Arg::with_name("attach")
                .short('a')
                .long("attach")
                .takes_value(true)
                .exclusive(true)
                .help("Attach to session with provided name."),
        )
        .arg(
            Arg::with_name("kill")
                .short('k')
                .long("kill")
                .takes_value(true)
                .exclusive(true)
                .help("Kill session with provided name."),
        )
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .takes_value(true)
                .exclusive(true)
                .help("List all sessions."),
        )
        .get_matches();

    matches.value_of("new").map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["new", "-s", session_name])
            .status()
            .unwrap()
    });
    matches.value_of("attach").map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["a", "-t", session_name])
            .status()
            .unwrap()
    });
    matches.value_of("kill").map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["kill-session", "-t", session_name])
            .status()
            .unwrap()
    });
    matches.value_of("list").map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["kill-session", "-t", session_name])
            .status()
            .unwrap()
    });
}
