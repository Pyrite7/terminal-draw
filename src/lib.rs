use std::io::Write;



pub mod geometry;

use geometry::*;
pub use geometry::{Vec2, Rect};
pub use nu_ansi_term::{Style, Color};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnsiChar(pub char, pub Style);

impl From<char> for AnsiChar {
    fn from(value: char) -> Self {
        AnsiChar(value, Style::default().reset_before_style())
    }
}







#[derive(Clone, PartialEq, Eq, Debug)]
struct DrawState {
    string: String,
    cursor_pos: DefVec2,
    style: Style,
}

impl DrawState {
    fn new() -> Self {
        Self { string: "".to_string(), cursor_pos: (0,0), style: Style::default() }
    }
    fn reset_style(&mut self) {
        if self.style != Style::default() {
            self.style = Style::default();
            self.string.push_str("\x1B[0m");
        }
    }
    fn cursor_to(&mut self, pos: DefVec2) {
        if self.cursor_pos != pos {
            self.string.push_str(format!("\x1B[{};{}H", pos.y(), pos.x()).as_str());
            self.cursor_pos = pos;
        }
    }
    fn push(&mut self, char: AnsiChar) {
        self.string.push_str(format!("{}{}", char.1.prefix(), char.0).as_str());

        self.style = char.1;
        self.cursor_pos.0 += 1;
    }
}


/// Prints a rectangle of styled Unicode characters to the given Writer.
/// Note that the writer has to be flushed after this.
pub fn draw_to(writer: &mut impl Write, area: impl Rect, mut contents: impl FnMut(DefVec2) -> AnsiChar) -> std::io::Result<()> {
    let mut state = DrawState::new();

    state.reset_style();

    for row in area.top()..=area.bottom() {
        state.cursor_to((area.left(), row));

        for col in area.left()..=area.right() {
            state.push(contents((col - area.left(), row - area.top())));
        }
    }
    
    state.reset_style();

    writer.write_all(state.string.as_bytes())?;
    Ok(())
}




/// Sends an ANSI command to clear all text from the terminal and resets the cursor position and text style.
/// Note that the writer has to be flushed after this.
pub fn clear_all(writer: &mut impl Write) -> std::io::Result<()> {
    writer.write_all(b"\x1B[2J\x1B[H\x1B[0m")?;
    Ok(())
}

/// Sends an ANSI command to move the cursor to a position. Useful after calling draw() to move the terminal prompt to a desired location.
/// Note that the writer has to be flushed after this.
pub fn move_cursor_to(writer: &mut impl Write, pos: impl Vec2) -> std::io::Result<()> {
    writer.write_all(format!("\x1B[{};{}H", pos.y(), pos.x()).as_bytes())?;
    Ok(())
}

/// Clears a rectangular area of the terminal.
/// Note that the writer has to be flushed after this.
pub fn clear_area(writer: &mut impl Write, area: impl Rect) -> std::io::Result<()> {
    draw_to(writer, area, |_| ' '.into())?;
    Ok(())
}







