use clap::{Args, CommandFactory, Parser, Subcommand};

mod add;
mod branch;
mod checkout;
mod commit;
mod config;
mod git_operations;
mod log;
mod rebase;
mod revert;
mod stash;

#[derive(Parser)]
#[command(name = "gq", version = env!("CARGO_PKG_VERSION"), about = "GitQuick: Simplify your git workflow")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct ConfigArgs {
    key: String,
    value: String,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    #[command(about = "Save your local modifications to a new stash")]
    Push,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get and set options")]
    Config(ConfigArgs),
    #[command(about = "Add contents of new or changed files to the index")]
    Add,
    #[command(about = "Record changes to the repository")]
    Commit {
        #[arg(long = "fixup", help = "Create a new branch")]
        fixup: bool,
        #[arg(long = "amend", help = "Modify most recent commit")]
        amend: bool,
    },
    #[command(about = "List, create, or delete branches")]
    Branch {
        #[arg(short = 'd', long = "delete", help = "Delete a branch")]
        delete: bool,
        #[arg(short = 'D', help = "Force delete a branch")]
        force_delete: bool,
    },
    #[command(about = "Switch branches or restore working tree files")]
    Checkout {
        #[arg(short = 'b', long = "branch", help = "Create a new branch")]
        create_new: bool,
    },
    #[command(about = "Revert some existing commits")]
    Revert,
    #[command(about = "List commits")]
    Log,
    #[command(about = "Reapply commits on top of another base tip")]
    Rebase {
        #[arg(
            short = 'i',
            long = "interactive",
            help = "Let user edit list before rebasing"
        )]
        interactive: bool,
    },
    #[command(about = "Stash the changes in a dirty working directory away")]
    Stash(StashArgs),
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config();
    let result = match &cli.command {
        Some(Commands::Commit { fixup, amend }) => {
            commit::run_commit(config.commit, *fixup, *amend)
        }
        Some(Commands::Branch {
            delete,
            force_delete,
        }) => branch::run_branch(*delete, *force_delete),
        Some(Commands::Checkout { create_new }) => checkout::run_checkout(*create_new),
        Some(Commands::Revert) => revert::run_revert(),
        Some(Commands::Config(args)) => config::run_config(args),
        Some(Commands::Add) => add::stage_files(),
        Some(Commands::Log) => log::run_log(),
        Some(Commands::Rebase { interactive }) => rebase::run_rebase(*interactive),
        Some(Commands::Stash(args)) => match args.command {
            Some(StashCommands::Push) => stash::run_stash(true),
            None => stash::run_stash(false),
        },
        None => {
            Cli::command().print_help().unwrap();
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }
}
