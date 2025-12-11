use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::auth::AuthController;
use crate::ui;

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    EnteringApiKey,
    Authenticated,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    state: AppState,
    auth_controller: AuthController,
    api_key_input: String,
    error_message: Option<String>,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let auth_controller = AuthController::new()?;

        let state = match auth_controller.get_api_key()? {
            Some(api_key) => {
                if auth_controller.validate_api_key(&api_key).is_ok() {
                    AppState::Authenticated
                } else {
                    AppState::EnteringApiKey
                }
            }
            None => AppState::EnteringApiKey,
        };

        Ok(Self {
            running: false,
            state,
            auth_controller,
            api_key_input: String::new(),
            error_message: None,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        match self.state {
            AppState::EnteringApiKey => {
                ui::render_input_screen(frame, &self.api_key_input, self.error_message.as_deref());
            }
            AppState::Authenticated => {
                ui::render_main_screen(frame);
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match self.state {
            AppState::EnteringApiKey => self.handle_api_key_input(key)?,
            AppState::Authenticated => self.handle_authenticated_input(key)?,
        }
        Ok(())
    }

    fn handle_api_key_input(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Enter) => self.submit_api_key()?,
            (_, KeyCode::Backspace) => {
                self.api_key_input.pop();
                self.error_message = None;
            }
            (_, KeyCode::Char(c)) => {
                self.api_key_input.push(c);
                self.error_message = None;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_authenticated_input(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
        Ok(())
    }

    fn submit_api_key(&mut self) -> color_eyre::Result<()> {
        if self.api_key_input.is_empty() {
            self.error_message = Some("API key cannot be empty".to_string());
            return Ok(());
        }

        match self.auth_controller.set_api_key(self.api_key_input.clone()) {
            Ok(_) => {
                self.state = AppState::Authenticated;
                self.api_key_input.clear();
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Error: {}", e));
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
