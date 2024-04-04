use std::env;

use clap::{Parser, Subcommand};

/// Typst's test runner.
#[derive(Debug, Clone, Parser)]
#[clap(name = "typst-test", author)]
pub struct CliArguments {
    /// The command to run.
    #[command(subcommand)]
    pub command: Option<Command>,
    /// All the tests that contains the filter string will be run.
    pub filter: Vec<String>,
    /// Whether to update the reference hashes of non-passing tests.
    #[arg(short, long, default_value_t = env::var_os("UPDATE_EXPECT").is_some())]
    pub update: bool,
    /// Prevents the terminal from being cleared of test names.
    #[arg(short, long)]
    pub verbose: bool,
    /// How many threads to spawn when running the tests.
    #[arg(long)]
    pub num_threads: Option<usize>,
}

/// What to do.
#[derive(Debug, Clone, Subcommand)]
#[command()]
pub enum Command {
    /// Produces a comparison report between test results of two revisions.
    Compare(CompareCommand),
    /// Clears the on-disk test artifact store.
    Clean,
}

/// Produces a comparison report between test results of two revisions.
#[derive(Debug, Clone, Parser)]
pub struct CompareCommand {
    /// The old revision.
    #[clap(default_value = "TREE")]
    pub old: String,
    /// The new revision.
    #[clap(default_value = "LIVE")]
    pub new: String,
    /// Whether to automatically open the comparison report.
    #[arg(long = "open")]
    pub open: bool,
}
