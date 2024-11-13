use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tracing::info;

use crate::{action::Action, app::Mode, config::Config};

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
            Action::NormalMode => {
                self.current_mode = Mode::Normal;
                Ok(None)
            }
            Action::EditingMode => {
                self.current_mode = Mode::Editing;
                Ok(None)
            }
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
        //info!("{:#?}", partitions_vec.len());
        //
        let vert =
            Layout::vertical([Constraint::Percentage(92), Constraint::Percentage(8)]).split(area);
        //Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)]).split(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
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
            frame.render_widget(
                Paragraph::new(Text::from(Line::from(
                    keys.iter()
                        .map(|k| {
                            Span::from(format!("{} - {}", k.code, action))
                            //.bg(Color::from_u32(0x00111111 * i as u32))
                        })
                        .collect::<Vec<Span>>(),
                )))
                .block(
                    Block::new()
                        .style(Style::new().bg(Color::Black))
                        .padding(Padding::new(0, 0, 2, 0)),
                )
                .alignment(Alignment::Left),
                chunks[i],
            )
        }
        Ok(())
    }
}
