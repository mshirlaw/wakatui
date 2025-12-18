use ratatui::{
    Frame,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn render_input_screen(frame: &mut Frame, api_key_input: &str, error_message: Option<&str>) {
    let title = Line::from("WakaTUI - Setup").bold().blue().centered();

    let mut text = vec![
        Line::from(""),
        Line::from("Welcome to WakaTUI!").bold().centered(),
        Line::from(""),
        Line::from("No WakaTime API key found.").centered(),
        Line::from(""),
        Line::from(vec![
            Span::raw("Get your API key from: "),
            Span::styled(
                "https://wakatime.com/settings/api-key",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ])
        .centered(),
        Line::from(""),
        Line::from(""),
        Line::from("Enter your API key below:").bold().centered(),
        Line::from("(It will be securely stored in your system keychain)").centered(),
        Line::from(""),
    ];

    let masked_input = if api_key_input.is_empty() {
        Span::styled(
            "Type your API key here...",
            Style::default().fg(Color::DarkGray),
        )
    } else {
        Span::styled(
            "*".repeat(api_key_input.len()),
            Style::default().fg(Color::Green),
        )
    };

    text.push(Line::from(vec![Span::raw("  "), masked_input]));

    text.push(Line::from(""));
    text.push(Line::from(""));

    if let Some(error) = error_message {
        text.push(
            Line::from(Span::styled(
                error.to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))
            .centered(),
        );
        text.push(Line::from(""));
    }

    text.push(Line::from(""));
    text.push(
        Line::from(vec![
            Span::raw("Press "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to submit, "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to quit"),
        ])
        .centered(),
    );

    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered().title(title))
            .centered(),
        frame.area(),
    )
}
