use crate::args::{Cli, Commands};
use crate::new::run_new;
use clap::Parser;
use color_eyre::eyre::WrapErr;
use ratatui::{TerminalOptions, Viewport};

mod args;
mod new;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(24),
    });

    let cli = Cli::parse();

    match cli.subcommand {
        Commands::New(new) => run_new(new, terminal).await.wrap_err("Running New")?,
    }
    
    ratatui::restore();

    Ok(())
}
