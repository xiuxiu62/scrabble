use std::fmt::Display;

pub type Position = Vec2<usize>;

#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr) => {
        Position::new($x, $y)
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Copy + Display>([T; 2]);

impl<T: Copy + Display> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }

    #[inline]
    pub const fn x(&self) -> T {
        self.0[0]
    }

    #[inline]
    pub const fn y(&self) -> T {
        self.0[1]
    }

    #[inline]
    pub fn set_x(&mut self, value: T) {
        self.0[0] = value;
    }

    #[inline]
    pub fn set_y(&mut self, value: T) {
        self.0[1] = value;
    }
}

impl<T: Copy + Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}
