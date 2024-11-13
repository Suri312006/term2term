use clap::builder::styling::Style;
use color_eyre::{owo_colors::OwoColorize, Result};
use layout::Flex;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{fps::FpsCounter, Component};
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    fps_counter: FpsCounter,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let area = center(area, Constraint::Percentage(50), Constraint::Percentage(50));
        frame.render_widget(Paragraph::new("hello world"), area);

        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_set(Default::default())
                .fg(Color::Blue)
                .bg(Color::Rgb(0, 0, 0)),
            area,
        );
        self.fps_counter.draw(frame, area)?;

        Ok(())
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
