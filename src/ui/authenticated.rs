use ratatui::{
    Frame,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn render_main_screen(frame: &mut Frame) {
    let title = Line::from("WakaTUI").bold().blue().centered();

    let text = vec![
        Line::from(""),
        Line::from("Successfully authenticated!")
            .bold()
            .green()
            .centered(),
        Line::from(""),
        Line::from("Your WakaTime statistics will appear here.").centered(),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled(
                "q",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to quit"),
        ])
        .centered(),
    ];

    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered().title(title))
            .centered(),
        frame.area(),
    )
}
