use ratatui::{
    Frame,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn render_intro_screen(frame: &mut Frame) {
    let title = Line::from("WakaTUI").bold().blue().centered();
    let text = vec![
        Line::from(""),
        Line::from("Welcome to WakaTUI!"),
        Line::from(""),
        Line::from("This app needs to connect to your WakaTime account"),
        Line::from("to display your coding statistics"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
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
