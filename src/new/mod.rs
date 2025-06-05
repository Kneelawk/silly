use clap::Args;
use color_eyre::eyre::WrapErr;
use ratatui::DefaultTerminal;
use std::path::PathBuf;
use ratatui::widgets::Block;
use url::Url;

#[derive(Debug, Clone, Args)]
#[clap()]
pub struct NewArgs {
    /// The location of a repository. A repository can be specified with a full url or with a slash
    /// notation like 'Kneelawk/silly' that would match the first repository available in the list
    /// of known git hosts.
    pub repo: String,

    /// The path to where the new project should be created. To create the project in the current
    /// directory, use '.' for the current directory.
    pub path: PathBuf,

    /// The branch to checkout. Uses the repository's main branch if none is specified.
    #[arg(short, long)]
    pub branch: Option<String>,

    /// The host to get the git repository from. Searches the user's configured list of repositories
    /// if none is specified. If <REPO> is a full url then this is ignored.
    #[arg(long)]
    pub host: Option<Url>,
}

pub async fn run_new(new_args: NewArgs, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let full_repo;
    if let Ok(url) = Url::parse(&new_args.repo) {
        full_repo = url;
    } else if let Some(host) = &new_args.host {
        full_repo = host.join(&new_args.repo).wrap_err("Parsing repo url")?;
    } else {
        // TODO: replace with actual list
        full_repo = Url::parse("https://github.com/")
            .unwrap()
            .join(&new_args.repo)
            .wrap_err("Parsing repo url")?;
    }

    println!(
        " => Cloning from '{}' into '{}'",
        &full_repo, new_args.path.to_string_lossy()
    );

    terminal.draw(|frame| {
        let block = Block::new().title("Test");
        frame.render_widget(block, frame.area());
    }).wrap_err("Drawing to terminal")?;

    Ok(())
}
