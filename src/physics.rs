use core::ops::{Add, Sub, Mul, Div};

pub const GRAVITY: Acceleration = Acceleration(9.8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meter(pub f64);
// ヤードポンド殺すべし 慈悲はない
impl Meter {
    pub fn new(value: f64) -> Self {
        Meter(value)
    }
}

impl Add for Meter {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Meter(self.0 + other.0)
    }
}

impl Sub for Meter {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Meter(self.0 - other.0)
    }
}

impl Mul for Meter {
    type Output = SquareMeter;

    fn mul(self, other: Self) -> SquareMeter {
        SquareMeter(self.0 * other.0)
    }
}

impl Mul<f64> for Meter {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Meter(self.0 * scalar)
    }
}

impl Div<f64> for Meter {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Meter(self.0 / scalar)
    }
}

impl Div<Second> for Meter {
    type Output = Velocity;

    fn div(self, time: Second) -> Velocity {
        Velocity(self.0 / time.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareMeter(pub f64);

impl SquareMeter {
    pub fn new(value: f64) -> Self {
        SquareMeter(value)
    }
}

impl Add for SquareMeter {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        SquareMeter(self.0 + other.0)
    }
}

impl Sub for SquareMeter {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        SquareMeter(self.0 - other.0)
    }
}

impl Div for SquareMeter {
    type Output = Meter;

    fn div(self, other: Self) -> Meter {
        Meter(self.0 / other.0)
    }
}

impl Mul<f64> for SquareMeter {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        SquareMeter(self.0 * scalar)
    }
}

impl Div<f64> for SquareMeter {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        SquareMeter(self.0 / scalar)
    }
}

impl Div<Meter> for SquareMeter {
    type Output = Meter;

    fn div(self, other: Meter) -> Meter {
        Meter(self.0 / other.0)
    }
}




#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Second(pub f64);

impl Second {
    pub fn new(value: f64) -> Self {
        Second(value)
    }
}

impl Add for Second {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Second(self.0 + other.0)
    }
}

impl Sub for Second {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Second(self.0 - other.0)
    }
}

impl Mul<f64> for Second {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Second(self.0 * scalar)
    }
}

impl Div<f64> for Second {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Second(self.0 / scalar)
    }
}

impl Div<Second> for Second {
    type Output = f64;

    fn div(self, other: Second) -> f64 {
        self.0 / other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub f64);

impl Velocity {
    pub fn new(value: f64) -> Self {
        Velocity(value)
    }
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Velocity(self.0 + other.0)
    }
}

impl Sub for Velocity {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Velocity(self.0 - other.0)
    }
}

impl Mul<f64> for Velocity {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Velocity(self.0 * scalar)
    }
}

impl Mul<Second> for Velocity {
    type Output = Meter;

    fn mul(self, time: Second) -> Meter {
        Meter(self.0 * time.0)
    }
}

impl Div<f64> for Velocity {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Velocity(self.0 / scalar)
    }
}

impl Div<Second> for Velocity {
    type Output = Acceleration;

    fn div(self, time: Second) -> Acceleration {
        Acceleration(self.0 / time.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration(pub f64);

impl Acceleration {
    pub fn new(value: f64) -> Self {
        Acceleration(value)
    }
}

impl Add for Acceleration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Acceleration(self.0 + other.0)
    }
}

impl Sub for Acceleration {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Acceleration(self.0 - other.0)
    }
}

impl Mul<f64> for Acceleration {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Acceleration(self.0 * scalar)
    }
}

impl Div<f64> for Acceleration {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Acceleration(self.0 / scalar)
    }
}

impl Mul<Second> for Acceleration {
    type Output = Velocity;

    fn mul(self, time: Second) -> Velocity {
        Velocity(self.0 * time.0)
    }
}

impl Mul<Kilogram> for Acceleration {
    type Output = Newton;

    fn mul(self, mass: Kilogram) -> Newton {
        Newton(self.0 * mass.0)
    }
}



// mass
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilogram(pub f64);

impl Kilogram {
    pub fn new(value: f64) -> Self {
        Kilogram(value)
    }
}

impl Add for Kilogram {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Kilogram(self.0 + other.0)
    }
}

impl Sub for Kilogram {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Kilogram(self.0 - other.0)
    }
}

impl Mul<f64> for Kilogram {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Kilogram(self.0 * scalar)
    }
}

impl Div<f64> for Kilogram {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Kilogram(self.0 / scalar)
    }
}

// force
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Newton(pub f64);

impl Newton {
    pub fn new(value: f64) -> Self {
        Newton(value)
    }
}

impl Add for Newton {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Newton(self.0 + other.0)
    }
}

impl Sub for Newton {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Newton(self.0 - other.0)
    }
}

impl Mul<f64> for Newton {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Newton(self.0 * scalar)
    }
}

impl Div<f64> for Newton {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Newton(self.0 / scalar)
    }
}

impl Div<Kilogram> for Newton {
    type Output = Acceleration;

    fn div(self, mass: Kilogram) -> Acceleration {
        Acceleration(self.0 / mass.0)
    }
}

// pressure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pascal(pub f64);

impl Pascal {
    pub fn new(value: f64) -> Self {
        Pascal(value)
    }
}

impl Add for Pascal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pascal(self.0 + other.0)
    }
}

impl Sub for Pascal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Pascal(self.0 - other.0)
    }
}

impl Mul<f64> for Pascal {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Pascal(self.0 * scalar)
    }
}

impl Div<f64> for Pascal {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Pascal(self.0 / scalar)
    }
}

impl Div<Meter> for Pascal {
    type Output = Newton;

    fn div(self, length: Meter) -> Newton {
        Newton(self.0 * length.0)
    }
}





