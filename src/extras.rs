use crate::*;





pub mod text_box {
    use crate::*;

    pub fn wrap_text(text: &str, width: u16) -> Vec<String> {
        let mut lines = text.lines();
        let mut res = Vec::new();
        loop {
            let Some(line) = lines.next() else {
                break res;
            };
            let mut line = line.to_string();
    
            while !line.is_empty() {
                res.push(line.chars().take(width as usize).collect());
                line = line.chars().skip(width as usize).collect();
            }
        }
    }
    
    pub fn draw_line(pos: impl Vec2, text: &str) {
        draw(DefRect::new(pos.x(), pos.y(), text.chars().count() as u16, 1), |(x, _)| {
            text.chars().nth(x as usize).unwrap_or('X').into()
        });
    }
    
    pub fn draw_text_box(area: impl Rect, text: &str) {
        let wrapped = wrap_text(text, area.w());
        draw(area, |(x, y)| {
            let Some(s) = wrapped.get(y as usize) else {
                return ' '.into();
            };
    
            s.chars()
                .nth(x as usize)
                .unwrap_or(' ')
                .into()
        });
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn wrap_text_works() {
            let s1 = "lorem ipsum dolor lorem ipsum dolor";
            let s2 = "lorem ipsum\ndolor";
            let s3 = "lorem ipsum dolor\nlorem ipsum";
            
            assert_eq!(wrap_text(s1, 6), vec!["lorem ", "ipsum ", "dolor ", "lorem ", "ipsum ", "dolor"]);
            assert_eq!(wrap_text(s2, 11), vec!["lorem ipsum", "dolor"]);
            assert_eq!(wrap_text(s3, 12), vec!["lorem ipsum ", "dolor", "lorem ipsum"]);
        }
    }
    
}





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

/// Clears a rectangular area of the terminal.
pub fn clear_area(area: impl Rect) {
    draw(area, |_| ' '.into());
}





pub trait DrawableToPos {
    fn draw_to_pos(&self, pos: impl Vec2);
}

pub trait DrawableToArea {
    fn draw_to_area(&self, area: impl Rect);
}



















