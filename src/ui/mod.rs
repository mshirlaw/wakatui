mod screens;
mod widgets;

pub use screens::{render_input_screen, render_main_screen};

#[cfg(test)]
mod tests {
    use super::{render_input_screen, render_main_screen};
    use ratatui::{Frame, Terminal, backend::TestBackend, buffer::Buffer};

    const WIDTH: u16 = 80;
    const HEIGHT: u16 = 24;

    // Sets up a fake terminal using Ratatui's TestBackend and renders a UI into it
    // * Creates a test backend with a fixed width/height (80x24).
    // * Builds a Terminal using that backend.
    // * Calls the provided render_fn so it can draw onto the terminal.
    // * Finally, grabs the rendered buffer (a 2D grid of cells) and returns it.
    fn render_to_buffer<F>(render_fn: F) -> Buffer
    where
        F: FnOnce(&mut Frame<'_>),
    {
        let backend = TestBackend::new(WIDTH, HEIGHT);
        let mut terminal = Terminal::new(backend).expect("should create terminal");

        terminal
            .draw(|frame| {
                render_fn(frame);
            })
            .expect("should render frame");

        terminal.backend().buffer().clone()
    }

    // This converts the captured buffer into plain text so we can look for specific strings:
    // * Loops through every row (y) and column (x) of the buffer.
    // * Builds up each line using the characters in each cell (cell.symbol()).
    // * Trims trailing whitespace (so we don’t get huge empty borders).
    // * Joins all rows with newlines for a big string.
    fn buffer_to_string(buffer: Buffer) -> String {
        let mut lines = Vec::with_capacity(HEIGHT as usize);
        for y in 0..HEIGHT {
            let mut line = String::new();
            for x in 0..WIDTH {
                let cell = &buffer[(x, y)];
                line.push_str(cell.symbol());
            }
            lines.push(line.trim_end().to_string());
        }
        lines.join("\n")
    }

    fn render_input(api_key: &str, error: Option<&str>) -> String {
        let buffer = render_to_buffer(|frame| render_input_screen(frame, api_key, error));
        buffer_to_string(buffer)
    }

    fn render_main() -> String {
        let buffer = render_to_buffer(|frame| render_main_screen(frame));
        buffer_to_string(buffer)
    }

    #[test]
    fn input_screen_shows_placeholder_text_when_api_key_is_empty() {
        let screen = render_input("", None);
        assert!(screen.contains("Type your API key here..."));
        assert!(screen.contains("Enter your API key below:"));
    }

    #[test]
    fn input_screen_masks_api_key_input() {
        let api_key = "super-secret";
        let screen = render_input(api_key, None);
        let mask = "*".repeat(api_key.len());
        assert!(screen.contains(&mask));
        assert!(!screen.contains("Type your API key here..."));
    }

    #[test]
    fn input_screen_shows_error_messages() {
        let error = "Invalid API key";
        let screen = render_input("", Some(error));
        assert!(screen.contains(error));
    }

    #[test]
    fn main_screen_shows_success_state() {
        let screen = render_main();
        assert!(screen.contains("languages"));
    }
}
