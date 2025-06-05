use clap::{Parser, Subcommand};
use crate::new::NewArgs;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Creates a new project from a template
    New(NewArgs)
}
