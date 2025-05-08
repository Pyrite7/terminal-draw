#![allow(unused)]

use std::io::{stdout, Write};

use nu_ansi_term::Style;








#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnsiChar(pub char, pub Style);

impl From<char> for AnsiChar {
    fn from(value: char) -> Self {
        AnsiChar(value, Style::default().reset_before_style())
    }
}



pub trait Int: Sized {
    fn to_u16(&self) -> u16;
    fn from_u16(u16: u16) -> Self;
}

impl Int for u16 {
    fn to_u16(&self) -> u16 {
        *self
    }
    fn from_u16(u16: u16) -> Self {
        u16
    }
}

impl Int for u8 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as u8
    }
}
impl Int for u32 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as u32
    }
}
impl Int for u64 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as u64
    }
}
impl Int for u128 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as u128
    }
}
impl Int for usize {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as usize
    }
}

impl Int for i8 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as i8
    }
}
impl Int for i16 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as i16
    }
}
impl Int for i32 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as i32
    }
}
impl Int for i64 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as i64
    }
}
impl Int for i128 {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as i128
    }
}
impl Int for isize {
    fn to_u16(&self) -> u16 {
        *self as u16
    }
    fn from_u16(u16: u16) -> Self {
        u16 as isize
    }
}



pub trait Vec2: Sized {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn new(x: u16, y: u16) -> Self;

    fn from<T: Vec2>(t: &T) -> Self {
        Self::new(t.x(), t.y())
    }
    fn into<T: Vec2>(&self) -> T {
        T::from(self)
    }
}

impl<N1: Int, N2: Int> Vec2 for (N1, N2) {
    fn x(&self) -> u16 {
        self.0.to_u16()
    }
    fn y(&self) -> u16 {
        self.1.to_u16()
    }
    fn new(x: u16, y: u16) -> Self {
        (N1::from_u16(x), N2::from_u16(y))
    }
}

pub type DefVec2 = (u16, u16);

pub trait Rect: Sized {
    fn top(&self) -> u16;
    fn left(&self) -> u16;
    fn h(&self) -> u16;
    fn w(&self) -> u16;
    fn new(top: u16, left: u16, w: u16, h: u16) -> Self;

    fn bottom(&self) -> u16 {
        self.top() + self.h()
    }
    fn right(&self) -> u16 {
        self.left() + self.w()
    }

    fn top_left(&self) -> DefVec2 {
        (self.left(), self.top())
    }
    fn top_right(&self) -> DefVec2 {
        (self.right(), self.top())
    }
    fn bottom_left(&self) -> DefVec2 {
        (self.left(), self.bottom())
    }
    fn bottom_right(&self) -> DefVec2 {
        (self.right(), self.bottom())
    }

    fn from<T: Rect>(t: &T) -> Self {
        Self::new(t.top(), t.left(), t.w(), t.h())
    }
    fn into<T: Rect>(&self) -> T {
        T::from(self)
    }
}

impl<V1: Vec2, V2: Vec2> Rect for (V1, V2) {
    fn top(&self) -> u16 {
        self.0.y().min(self.1.y())
    }
    fn left(&self) -> u16 {
        self.0.x().min(self.1.x())
    }
    fn h(&self) -> u16 {
        self.0.y().max(self.1.y()) - self.top()
    }
    fn w(&self) -> u16 {
        self.0.x().max(self.1.x()) - self.top()
    }
    fn new(top: u16, left: u16, w: u16, h: u16) -> Self where Self: Sized {
        (V1::new(left, top), V2::new(left + w, top + h))
    }
}

pub type DefRect = (DefVec2, DefVec2);





/// Clears all text from the terminal and resets the cursor position and text style.
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[H\x1B[0m");
    stdout().flush().unwrap();
}

/// Moves the cursor to a position. Useful after calling draw() to move the terminal prompt to a desired location.
pub fn move_cursor_to(pos: impl Vec2) {
    print!("\x1B[{};{}H", pos.y(), pos.x());
    stdout().flush().unwrap();
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

    for row in area.top()..area.bottom() {
        state.cursor_to((area.left(), row));

        for col in area.left()..area.right() {
            state.push(contents((col - area.left(), row - area.top())));
        }
    }
    
    state.reset_style();

    println!("{}", state.string);
}









