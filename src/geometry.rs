

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
    fn left(&self) -> u16;
    fn top(&self) -> u16;
    fn h(&self) -> u16;
    fn w(&self) -> u16;
    fn new(left: u16, top: u16, w: u16, h: u16) -> Self;

    fn bottom(&self) -> u16 {
        self.top() + self.h() - 1
    }
    fn right(&self) -> u16 {
        self.left() + self.w() - 1
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
        Self::new(t.left(), t.top(), t.w(), t.h())
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
        self.0.y().max(self.1.y()) - self.top() + 1
    }
    fn w(&self) -> u16 {
        self.0.x().max(self.1.x()) - self.left() + 1
    }
    fn new(left: u16, top: u16, w: u16, h: u16) -> Self where Self: Sized {
        (V1::new(left, top), V2::new(left + w - 1, top + h - 1))
    }
}

pub type DefRect = (DefVec2, DefVec2);


#[cfg(test)]
mod geometry_tests {
    use super::*;

    #[test]
    fn rect_works() {
        let r1 = DefRect::new(1, 2, 1, 1);
        let r2 = DefRect::new(3, 5, 2, 2);
        
        assert_eq!(r1, ((1, 2), (1, 2)));
        assert_eq!(r2, ((3, 5), (4, 6)));
    }

    #[test]
    fn rect_size() {
        let r1 = DefRect::new(1, 2, 1, 1);

        assert_eq!(r1.w(), 1);
        assert_eq!(r1.h(), 1);
        assert_eq!(r1.bottom_right(), (1, 2));
    }
}

