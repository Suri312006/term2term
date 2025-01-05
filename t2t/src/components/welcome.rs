use std::rc::Rc;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, info};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::Component;
use crate::{
    action::{Action, Mode, Selection},
    config::Config,
    utils::center,
};

#[derive(Default)]
pub struct Welcome {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    state_handler: PageStateHandler,
    mode: Mode,
}

impl Welcome {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
struct PageStateHandler {
    inner: PageState,
}

#[derive(Default, Clone)]
enum PageState {
    #[default]
    WelcomePage,
    NameAndSuffix(NameSuffixInput),
    RedirectToHome,
}

#[derive(Default, Clone)]
struct NameSuffixInput {
    name: Input,
    suffix: Input,
    active: Option<ActiveInput>,
    cached: Option<ActiveInput>,
}

#[derive(Default, Clone, Copy)]
enum ActiveInput {
    #[default]
    Name,
    Suffix,
}

impl PageStateHandler {
    fn next(&mut self) {
        match self.inner {
            PageState::WelcomePage => {
                self.inner = PageState::NameAndSuffix(NameSuffixInput::default());
            }
            PageState::NameAndSuffix(_) => {
                self.inner = PageState::RedirectToHome;
            }
            _ => {}
        }
    }
}

const TITLE: &str = r#"
 _____                     ____    _____
|_   _|__ _ __ _ __ ___   |___ \  |_   _|__ _ __ _ __ ___
  | |/ _ \ '__| '_ ` _ \    __) |   | |/ _ \ '__| '_ ` _ \
  | |  __/ |  | | | | | |  / __/    | |  __/ |  | | | | | |
  |_|\___|_|  |_| |_| |_| |_____|   |_|\___|_|  |_| |_| |_|
        "#;

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
            Action::Enter => match &mut self.state_handler.inner {
                PageState::WelcomePage => {
                    self.command_tx
                        .clone()
                        .expect("Expected to have command transmitter")
                        .send(Action::Mode(Mode::Editing))?;
                    self.state_handler.next();

                    //if let PageState::NameAndSuffix(x) = &mut self.state_handler.inner {
                    //    x.active = Some(ActiveInput::Name);
                    //}
                }
                PageState::NameAndSuffix(inputs) => {
                    //TODO: We need to verify that whatever they entered is valid
                    //and then we need to send off a register request to the client
                    // and then we switch to redirect to home
                    //
                    info!("got name: {}", inputs.name.value());
                    info!("got suffix: {}", inputs.suffix.value());
                }
                PageState::RedirectToHome => {
                    // here we are just checking the receive on the client that something came
                    // back!
                }
            },

            Action::Mode(m) => match m {
                Mode::Editing => {
                    if let PageState::NameAndSuffix(x) = &mut self.state_handler.inner {
                        if x.cached.is_none() {
                            x.active = Some(ActiveInput::Name);
                        } else {
                            x.active = x.cached;
                        }
                    }
                    self.mode = Mode::Editing;
                }
                Mode::Normal => {
                    if let PageState::NameAndSuffix(x) = &mut self.state_handler.inner {
                        x.cached = x.active;
                        x.active = None;
                    }
                    self.mode = Mode::Normal;
                }
            },

            Action::Selection(sel) => {
                if let PageState::NameAndSuffix(x) = &mut self.state_handler.inner {
                    match sel {
                        Selection::Name => {
                            x.active = Some(ActiveInput::Name);
                        }
                        Selection::Suffix => {
                            x.active = Some(ActiveInput::Suffix);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match self.mode {
            Mode::Normal => {
                match &mut self.state_handler.inner {
                    PageState::NameAndSuffix(inputs) => match key.code {
                        // switches the current active input
                        KeyCode::Tab => {
                            match inputs.active {
                                Some(ActiveInput::Name) => {
                                    inputs.active = Some(ActiveInput::Suffix)
                                }
                                Some(ActiveInput::Suffix) => {
                                    inputs.active = Some(ActiveInput::Name)
                                }
                                _ => {}
                            };
                        }

                        KeyCode::Char('1') => inputs.active = Some(ActiveInput::Name),
                        KeyCode::Char('2') => inputs.active = Some(ActiveInput::Suffix),
                        _ => {}
                    },
                    _ => {}
                }
            }

            Mode::Editing => {
                if let PageState::NameAndSuffix(inputs) = &mut self.state_handler.inner {
                    match inputs.active {
                        Some(ActiveInput::Name) => {
                            inputs.name.handle_event(&crossterm::event::Event::Key(key));
                        }
                        Some(ActiveInput::Suffix) => {
                            inputs
                                .suffix
                                .handle_event(&crossterm::event::Event::Key(key));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(None)

        //TODO: want to make sure its not editing when we are in typing mode
        // match &mut self.state_handler.inner {
        //     PageState::NameAndSuffix(inputs) => match key.code {
        //         // switches the current active input
        //         KeyCode::Tab => {
        //             match inputs.active {
        //                 Some(ActiveInput::Name) => inputs.active = Some(ActiveInput::Suffix),
        //                 Some(ActiveInput::Suffix) => inputs.active = Some(ActiveInput::Name),
        //                 _ => {}
        //             };
        //             Ok(None)
        //         }

        //         _ => {
        //             match inputs.active {
        //                 Some(ActiveInput::Name) => {
        //                     inputs.name.handle_event(&crossterm::event::Event::Key(key));
        //                 }
        //                 Some(ActiveInput::Suffix) => {
        //                     inputs
        //                         .suffix
        //                         .handle_event(&crossterm::event::Event::Key(key));
        //                 }
        //                 _ => {}
        //             }
        //             Ok(None)
        //         }
        //     },
        //     _ => Ok(None),
        // }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        //let area = center(area, Constraint::Percentage(50), Constraint::Percentage(50));
        //frame.render_widget(Paragraph::new("hello world"), area);
        match &self.state_handler.inner {
            PageState::WelcomePage => {
                //let block = Block::bordered()
                //    .border_type(BorderType::Plain)
                //    .border_set(Default::default())
                //    .fg(Color::White);
                //frame.render_widget(block, area);

                let welcome_block =
                    center(area, Constraint::Percentage(50), Constraint::Percentage(50));

                let title_p = Paragraph::new(Text::from(TITLE).style(Style::new().bold()))
                    .block(
                        Block::new()
                            .style(Style::new().bg(Color::Black))
                            .padding(Padding::new(0, 0, welcome_block.height / 3, 0)),
                    )
                    .style(Style::new().white())
                    .alignment(Alignment::Center);

                frame.render_widget(title_p, welcome_block);

                //let t = Paragraph::new(title);

                let p = Paragraph::new(Text::from(vec![
                    Line::from("Press Enter To Begin").style(Style::new().slow_blink())
                ]))
                .block(
                    Block::new()
                        .style(Style::new().bg(Color::Black))
                        .padding(Padding::new(
                            0,                            // left
                            0,                            // right
                            2 * welcome_block.height / 3, // top
                            0,                            // bottom
                        )),
                )
                .style(Style::new().white())
                .alignment(Alignment::Center);

                //let enter_to_start = Paragraph::new("Press Enter to Begin")
                //    .style(Style::new().white().slow_blink())
                //    .alignment(Alignment::Center);

                frame.render_widget(p, welcome_block);
                //frame.render_widget(enter_to_start, welcome_block);
            }
            PageState::NameAndSuffix(inputs) => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical) // this is the direction the rows form
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(1), // first row
                            Constraint::Length(3), // second row
                            Constraint::Min(1),    // last chunk
                            Constraint::Min(1),    // last chunk
                        ]
                        .as_ref(),
                    )
                    .split(area);

                let name = Paragraph::new(inputs.name.value())
                    .block(Block::default().borders(Borders::ALL).title("Name"));
                let suffix = Paragraph::new(inputs.suffix.value())
                    .block(Block::default().borders(Borders::ALL).title("Suffix"));
                frame.render_widget(name, chunks[1]);
                frame.render_widget(suffix, chunks[2]);

                let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for curs
                match inputs.active {
                    Some(ActiveInput::Name) => {
                        let scroll = inputs.name.visual_scroll(width as usize);
                        frame.set_cursor_position((
                            // Put cursor past the end of the input text
                            chunks[1].x
                                + ((inputs.name.visual_cursor()).max(scroll) - scroll) as u16
                                + 1,
                            // Move one line down, from the border to the input line
                            chunks[1].y + 1,
                        ))
                    }
                    Some(ActiveInput::Suffix) => {
                        let scroll = inputs.suffix.visual_scroll(width as usize);
                        frame.set_cursor_position((
                            // Put cursor past the end of the input text
                            chunks[2].x
                                + ((inputs.suffix.visual_cursor()).max(scroll) - scroll) as u16
                                + 1,
                            // Move one line down, from the border to the input line
                            chunks[2].y + 1,
                        ))
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
