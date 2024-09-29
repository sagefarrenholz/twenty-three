use std::ops;

#[derive(Debug, Clone, Default, Copy)]
pub struct Vec2(f32, f32);

#[macro_export]
macro_rules! v2 {
    ($x:expr, $y:expr) => {{
        Vec2::new(($x, $y))
    }};
}
impl Vec2 {
    pub fn new<T: Into<(f32, f32)>>(val: T) -> Self {
        let (a, b) = val.into();
        Self(a, b)
    }

    pub fn new_x<T: Into<f32>>(x: T) -> Self {
        Self(x.into(), 0.)
    }

    pub fn new_y<T: Into<f32>>(y: T) -> Self {
        Self(0., y.into())
    }

    pub fn add(&mut self, Vec2(x, y): Vec2) -> &mut Self {
        self.0 += x;
        self.1 += y;
        self
    }

    pub fn mul(&mut self, scalar: f32) -> &mut Self {
        self.0 *= scalar;
        self.1 *= scalar;
        self
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    pub fn mag(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }

    pub fn norm(mut self) -> Self {
        let mag = self.mag();
        if mag != 0. {
            self.0 /= mag;
            self.1 /= mag;
        }

        self
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vec2(x, y)
    }
}

impl<T: Into<f32>> ops::Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let scalar: f32 = rhs.into();
        Vec2(self.0 * scalar, self.1 * scalar)
    }
}

impl<'a, T: Into<f32>> ops::Mul<T> for &'a mut Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let scalar: f32 = rhs.into();
        Vec2(self.0 * scalar, self.1 * scalar)
    }
}

impl<'a, T: Into<f32>> ops::Mul<T> for &'a Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let scalar: f32 = rhs.into();
        Vec2(self.0 * scalar, self.1 * scalar)
    }
}

impl<'a, T: Into<Vec2>> ops::AddAssign<T> for &'a mut Vec2 {
    fn add_assign(&mut self, rhs: T) {
        let Vec2(x, y): Vec2 = rhs.into();
        self.0 += x;
        self.1 += y;
    }
}

impl<T: Into<Vec2>> ops::AddAssign<T> for Vec2 {
    fn add_assign(&mut self, rhs: T) {
        let Vec2(x, y): Vec2 = rhs.into();
        self.0 += x;
        self.1 += y;
    }
}

impl<T: Into<f32>> ops::MulAssign<T> for Vec2 {
    fn mul_assign(&mut self, rhs: T) {
        let rhs: f32 = rhs.into();
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
