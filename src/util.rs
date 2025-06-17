use ratatui::DefaultTerminal;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Paragraph, Widget};
use std::io::{BufRead, ErrorKind, Write};
use color_eyre::eyre::WrapErr;

pub struct Term {
    pub terminal: DefaultTerminal,
    last: String,
}

impl Term {
    pub fn new(terminal: DefaultTerminal) -> Term {
        Self {
            terminal,
            last: "".to_string(),
        }
    }
}

impl Write for Term {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        
        let mut v = buf.lines().collect::<Result<Vec<_>, _>>()?;
        let ends_newline = buf.ends_with("\n".as_bytes()) || buf.ends_with("\r\n".as_bytes());
        if v.len() > 1 || ends_newline {
            v.first_mut().unwrap().insert_str(0, &self.last);
            self.last.clear();
        }
        let end = if ends_newline {v.len()} else {v.len() - 1};
        let mut use_prev = true;
        for s in v {
            if use_prev {
                self.lines.last_mut().unwrap().push_str(&s);
            } else {
                self.lines.push(s);
            }
            use_prev = false;
        }
        if  {
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let lines = self
            .lines
            .drain(..self.lines.iter().len() - 1)
            .map(Line::from)
            .collect::<Vec<_>>();
        self.terminal.insert_before(lines.len() as u16, |buf| {
            Paragraph::new(Text::from(lines)).render(buf.area, buf);
        })?;

        Ok(())
    }
}
