use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::api::{ApiClient, DailySummary, DurationsResponse};
use crate::auth::AuthController;
use crate::ui;

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    EnteringApiKey,
    Authenticated,
}

#[derive(Debug, Default)]
pub struct AppData {
    pub today_summary: Option<DailySummary>,
    pub today_durations: Option<DurationsResponse>,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    state: AppState,
    auth_controller: AuthController,
    api_key_input: String,
    error_message: Option<String>,
    api_client: Option<ApiClient>,
    data: AppData,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let auth_controller = AuthController::new()?;

        let (state, api_client, data) = match auth_controller.get_api_key()? {
            Some(api_key) => {
                if auth_controller.validate_api_key(&api_key).is_ok() {
                    let client = ApiClient::new(api_key);
                    let data = Self::fetch_data(&client);
                    (AppState::Authenticated, Some(client), data)
                } else {
                    (AppState::EnteringApiKey, None, AppData::default())
                }
            }
            None => (AppState::EnteringApiKey, None, AppData::default()),
        };

        Ok(Self {
            running: false,
            state,
            auth_controller,
            api_key_input: String::new(),
            error_message: None,
            api_client,
            data,
        })
    }

    fn fetch_data(client: &ApiClient) -> AppData {
        let today_summary = client
            .get_today()
            .ok()
            .and_then(|r| r.data.into_iter().next());

        let today_durations = client.get_today_durations().ok();

        AppData {
            today_summary,
            today_durations,
        }
    }

    pub fn refresh_data(&mut self) {
        if let Some(client) = &self.api_client {
            self.data = Self::fetch_data(client);
        }
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
                ui::render_main_screen(frame, &self.data);
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
            (_, KeyCode::Char('r') | KeyCode::Char('R')) => self.refresh_data(),
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
                let client = ApiClient::new(self.api_key_input.clone());
                self.data = Self::fetch_data(&client);
                self.api_client = Some(client);
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
