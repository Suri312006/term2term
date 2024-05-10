use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};

use ratatui::prelude::*;


// type alias for terminal type
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

// init term
pub fn init() -> io::Result<Tui>{
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode();

    Terminal::new(CrosstermBackend::new(stdout()))
}


// restore term
pub fn restore() -> io::Result<()>{
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode();
    Ok(())

}




