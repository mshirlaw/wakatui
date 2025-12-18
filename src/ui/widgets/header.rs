use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Header;

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner = block.inner(area);
        block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        let left = vec![Line::from("left").bold().green().left_aligned()];
        Paragraph::new(left).render(chunks[0], buf);

        let right = vec![Line::from("right").bold().green().right_aligned()];
        Paragraph::new(right).render(chunks[1], buf);
    }
}
