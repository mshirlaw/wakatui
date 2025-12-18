use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Projects;

impl Widget for Projects {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        let first = vec![Line::from("languages").bold().green().centered()];

        Paragraph::new(first)
            .block(Block::bordered())
            .render(layout[0], buf);

        let second = vec![Line::from("projects").bold().green().centered()];
        Paragraph::new(second)
            .block(Block::bordered())
            .render(layout[1], buf);
    }
}
