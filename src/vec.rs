
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq)]
pub struct MathVec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> MathVec2<T> {
    pub fn new(x: T, y: T) -> Self {
        MathVec2 { x, y }
    }

}

impl<T>MathVec2<T> {
    pub fn norm(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();
        (x * x + y * y).sqrt()
    }
}

impl<T> Add for MathVec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MathVec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for MathVec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        MathVec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for MathVec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        MathVec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> AddAssign for MathVec2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> SubAssign for MathVec2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> MulAssign<T> for MathVec2<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }

}


impl From<mint::Vector2<f32>> for MathVec2<f32> {
    fn from(v: mint::Vector2<f32>) -> Self {
        MathVec2::new(v.x, v.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct MathVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


impl<T> MathVec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        MathVec3 { x, y, z }
    }
}

impl<T> Add for MathVec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MathVec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for MathVec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        MathVec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for MathVec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        MathVec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> AddAssign for MathVec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> SubAssign for MathVec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> MulAssign<T> for MathVec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl From<mint::Vector3<f32>> for MathVec3<f32> {
    fn from(v: mint::Vector3<f32>) -> Self {
        MathVec3::new(v.x, v.y, v.z)
    }
}