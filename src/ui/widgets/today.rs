use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Today;

impl Widget for Today {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner = block.inner(area);
        block.render(area, buf);

        let text = vec![Line::from("today").bold().green().left_aligned()];
        Paragraph::new(text).centered().render(inner, buf);
    }
}
