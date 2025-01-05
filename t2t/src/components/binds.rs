use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tracing::info;

use crate::{
    action::{Action, Mode, Selection},
    config::Config,
};

use super::Component;

#[derive(Default)]
pub struct BindsRow {
    config: Config,
    current_mode: Mode,
}

impl BindsRow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for BindsRow {
    fn register_config_handler(&mut self, config: Config) -> color_eyre::eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Mode(m) => match m {
                Mode::Normal => {
                    self.current_mode = Mode::Normal;
                    Ok(None)
                }
                Mode::Editing => {
                    self.current_mode = Mode::Editing;
                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        let partitions_vec = vec![
            Constraint::Length(18);
            self.config
                .keybindings
                .get(&self.current_mode)
                .expect("Should have keybinds set for current mode")
                .iter()
                .len()
        ];

        let vert =
            Layout::vertical([Constraint::Percentage(98), Constraint::Percentage(2)]).split(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&partitions_vec)
            .split(vert[1]);

        for (i, (keys, action)) in self
            .config
            .keybindings
            .get(&self.current_mode)
            .expect("expecting keybinds set for current mode")
            .iter()
            .enumerate()
        {
            let text = keys
                .iter()
                .map(|k| Span::from(format!("{} - {}", k.code, action)))
                .collect::<Vec<Span>>();

            let block = Block::new().style(Style::new().bg(Color::Black));

            frame.render_widget(
                Paragraph::new(Text::from(Line::from(text)))
                    .block(block)
                    .alignment(Alignment::Left),
                chunks[i],
            );
        }
        Ok(())
    }
}
