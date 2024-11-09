use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Welcome {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Welcome {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Welcome {
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
        //let area = center(area, Constraint::Percentage(50), Constraint::Percentage(50));
        //frame.render_widget(Paragraph::new("hello world"), area);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_set(Default::default())
            .fg(Color::Blue);
        //
        //let text = Text::from("Welcome to Term2Term!").centered();
        //let text = Text::from(vec![
        //    Line::from("hello world 1").left_aligned(),
        //    Line::from("hello world 2").style(Style::default().add_modifier(Modifier::SLOW_BLINK)),
        //    Line::from("hello world 3").right_aligned(),
        //])
        //.centered();
        //frame.render_widget(Paragraph::new(text).block(Block::bordered()), area);

        //frame.render_widget(
        //    Paragraph::new(text).block(block),
        //    //.title("penis"),
        //    area,
        //);

        Ok(())
    }
}
