use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

use std::io::Write;
use std::process::{Command, Stdio};
use translit::{self, FromLatin, Transliterator};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    output: String,
    transliterator: translit::Transliterator,
    /// Position of cursor in the editor area.
    character_index: usize,
}

fn set_clipboard(content: &str) {
    let mut child = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped()) // Pipe for sending content to xclip
        .spawn()
        .expect("Failed to start xclip");

    // Write the content to xclip's stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to xclip");
    }

    // Wait for xclip to complete
    child.wait().expect("Failed to wait on xclip");
}

impl App {
    fn new() -> Self {
        let mut table = translit::gost779b_ru();
        table.extend(vec![
            ("ь", "'"),
            ("ъ", "''"),
            ("ы", "y'"),
            ("Ы", "Y'"),
            ("в", "w"),
            ("В", "W"),
            ("э", "e'"),
            ("Э", "E'"),
            ("Х", "H"),
            ("х", "h"),
        ]);

        Self {
            input: String::new(),
            output: String::new(),
            transliterator: Transliterator::new(table),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn transliterate(&mut self) {
        self.output = self.transliterator.from_latin(&self.input)
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
        self.transliterate();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
        self.transliterate();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn handle_enter(&mut self) {
        // TODO: handle me
        set_clipboard(self.output.as_str());
        self.input.clear();
        self.reset_cursor();
        self.transliterate();
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => self.handle_enter(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
        let [help_area, input_area, output_area] = vertical.areas(frame.area());

        let (msg, style) = (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to copy to clipboard".into(),
            ],
            Style::default(),
        );
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default())
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let output = Paragraph::new(self.output.as_str())
            .style(Style::default())
            .block(Block::bordered().title("Output"));
        frame.render_widget(output, output_area);

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            input_area.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));
    }
}
