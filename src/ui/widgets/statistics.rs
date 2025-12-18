use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Statistics;

impl Widget for Statistics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(inner);

        let first = vec![Line::from("today").bold().green().centered()];
        Paragraph::new(first)
            .block(Block::bordered())
            .render(layout[0], buf);

        let second = vec![Line::from("week").bold().green().centered()];
        Paragraph::new(second)
            .block(Block::bordered())
            .render(layout[1], buf);

        let third = vec![Line::from("avg").bold().green().centered()];
        Paragraph::new(third)
            .block(Block::bordered())
            .render(layout[2], buf);

        let fourth = vec![Line::from("streak").bold().green().centered()];
        Paragraph::new(fourth)
            .block(Block::bordered())
            .render(layout[3], buf);
    }
}
