use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{
    io::{stdout, Result},
    task::Wake, vec,
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen);

    enable_raw_mode();

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;

    term.clear();

    //TODO: main loop
    loop {
        term.draw(|frame| {
            let area = frame.size();

            frame.render_widget(
                Paragraph::new("Hello alb!, press Q to quit")
                    .white()
                    .on_black(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('Q')
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;

    disable_raw_mode()?;

    Ok(())
}
