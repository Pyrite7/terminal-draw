use std::io::{stdout, Write};



pub mod extras;
pub mod geometry;

use geometry::*;
pub use geometry::{Vec2, Rect};
pub use extras::{DrawableToArea, DrawableToPos, clear_area, clear_terminal, move_cursor_to};
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


/// Prints a rectangle of styled Unicode characters to stdout.
pub fn draw(area: impl Rect, mut contents: impl FnMut(DefVec2) -> AnsiChar) {
    let mut state = DrawState::new();

    state.reset_style();

    for row in area.top()..=area.bottom() {
        state.cursor_to((area.left(), row));

        for col in area.left()..=area.right() {
            state.push(contents((col - area.left(), row - area.top())));
        }
    }
    
    state.reset_style();

    println!("{}", state.string);
}








