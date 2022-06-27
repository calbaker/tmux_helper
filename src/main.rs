use clap::{IntoApp, Parser, Subcommand};
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

#[derive(Subcommand)]
enum SubCommands {
    /// Create new session with provided name.
    New {
        #[clap(value_parser)]
        session: String,
    },
    /// Attach to session with provided name.
    Attach {
        #[clap(value_parser)]
        session: String,
    },
    /// Kill session with provided name.
    Kill {
        #[clap(value_parser)]
        session: String,
    },
    /// List all sessions.
    #[clap(value_parser)]
    List,
    /// Generate shell completions to put in ~/.bash_completion
    Completions {
        /// The shell to generate the completions for
        #[clap(arg_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    let tmux_args = TmuxArgs::parse();

    match tmux_args.command {
        SubCommands::New { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["new", "-s", &session])
                .status()
                .unwrap();
        }
        SubCommands::Attach { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["a", "-t", &session])
                .status()
                .unwrap();
        }
        SubCommands::Kill { session } => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["kill-session", "-t", &session])
                .status()
                .unwrap();
        }
        SubCommands::List => {
            ProcCommand::new("/usr/bin/tmux")
                .args(&["list-sessions"])
                .status()
                .unwrap();
        }
        // e.g. `$ cli completions bash`
        SubCommands::Completions { shell } => {
            shell.generate(&mut TmuxArgs::command(), &mut std::io::stdout());
        }
    }
}
