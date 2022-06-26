use clap::Parser;
use std::process::Command as ProcCommand;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Wrapper for tmux with simpler interface.
/// Use `<Ctrl>+b d` to disconnect from a session.  The session will remain active.   
/// Add to path by running the following or putting in .bashrc:   
/// `export PATH=/path/to/tmux_helper/folder:$PATH`
struct Tmux {
    #[clap(short, long, value_parser, exclusive = true)]
    /// Create new session with provided name.
    new: Option<String>,
    #[clap(short, long, value_parser, exclusive = true)]
    /// Attach to session with provided name.
    attach: Option<String>,
    /// Kill session with provided name.
    #[clap(short, long, value_parser, exclusive = true)]
    kill: Option<String>,
    /// List all sessions.
    #[clap(short, long, exclusive = true)]
    list: bool,
}

fn main() {
    let tmux = Tmux::parse();

    tmux.new.map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["new", "-s", &session_name])
            .status()
            .unwrap()
    });
    tmux.attach.map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["a", "-t", &session_name])
            .status()
            .unwrap()
    });
    tmux.kill.map(|session_name| {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["kill-session", "-t", &session_name])
            .status()
            .unwrap()
    });
    if tmux.list {
        ProcCommand::new("/usr/bin/tmux")
            .args(&["list-sessions"])
            .status()
            .unwrap();
    };
}
