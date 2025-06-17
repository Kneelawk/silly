use crate::util::Term;
use clap::Args;
use color_eyre::eyre::WrapErr;
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
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

pub async fn run_new(new_args: NewArgs, mut term: Term) -> color_eyre::Result<()> {
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

    let mut app = NewApp::new(full_repo, new_args.path, new_args.branch, &mut term)
        .wrap_err("Creating new app")?;

    while app.update(&mut term).await.wrap_err("Updating app")? {
        term.terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .wrap_err("Drawing to terminal")?;
    }

    Ok(())
}

struct NewApp {
    full_repo: Url,
    path: PathBuf,
    branch: Option<String>,
}

impl NewApp {
    fn new(
        full_repo: Url,
        path: PathBuf,
        branch: Option<String>,
        term: &mut Term,
    ) -> color_eyre::Result<NewApp> {
        writeln!(
            term,
            " => Cloning from '{}' into '{}'",
            &full_repo,
            path.to_string_lossy()
        )
        .wrap_err("Writing to term")?;

        Ok(Self {
            full_repo,
            path,
            branch,
        })
    }

    async fn update(&mut self, _term: &mut Term) -> color_eyre::Result<bool> {
        Ok(!event::poll(Duration::from_millis(20)).wrap_err("Polling input")?
            || !matches!(
                event::read().wrap_err("Reading input")?,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    ..
                })
            ))
    }
}

impl Widget for &NewApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Silly Template Generator ".bold());
        let instructions = Line::from(vec![" Exit ".into(), "<ESC>".blue().into()]);
        let block = Block::new()
            .title(title)
            .title_bottom(instructions)
            .border_set(border::ROUNDED);

        let clone_text = Text::from(vec![Line::from(vec![
            "Cloning from '".into(),
            format!("{}", &self.full_repo).green().into(),
            "' into '".into(),
            format!("{}", self.path.to_string_lossy()).green().into(),
            "'".into(),
        ])]);

        Paragraph::new(clone_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
