use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
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
        let title = Line::from("WakaTUI").bold().blue().centered();
        let text = vec![
            Line::from(""),
            Line::from("Welcome to WakaTUI!"),
            Line::from(""),
            Line::from("This app needs to connect to your WakaTime account"),
            Line::from("to display your coding statistics"),
            Line::from(""),
            Line::from(vec![
                ratatui::text::Span::raw("Press "),
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to authenticate with WakaTime"),
            ]),
            Line::from("or press Esc, Ctrl-C or q to quit"),
            Line::from(""),
            Line::from(Span::styled(
                "Your browser will open for authorization",
                Style::default().fg(Color::Yellow),
            )),
        ];
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
