use clap::{Parser, Subcommand};
use std::process::Command as ProcCommand;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Wrapper for tmux with simpler interface.
/// Use `<Ctrl>+b d` to disconnect from a session.  The session will remain active.   
/// Add to path by running the following or putting in .bashrc:   
/// `export PATH="/path/to/tmux_helper/folder":$PATH`.  You could put the file in
/// ~/.local/local/tmux_helper, for example.  
struct TmuxArgs {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Create new session with provided name.
    N {
        #[clap(value_parser)]
        session: String,
    },
    /// Attach to session with provided name.
    A {
        #[clap(value_parser)]
        session: String,
    },
    /// Kill session with provided name.
    K {
        #[clap(value_parser)]
        session: String,
    },
    /// List all sessions.
    #[clap(value_parser)]
    L,
}

fn main() {
    let tmux_args = TmuxArgs::parse();

    match tmux_args.command {
        SubCommands::N { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["new", "-s", &session])
                .status()
                .unwrap();
        }
        SubCommands::A { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["a", "-t", &session])
                .status()
                .unwrap();
        }
        SubCommands::K { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["kill-session", "-t", &session])
                .status()
                .unwrap();
        }
        SubCommands::L => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["list-sessions"])
                .status()
                .unwrap();
        }
    }
}
