#![allow(dead_code)]

use std::fmt::{Debug, Formatter};

/// Barometric pressure
/// (stored in microbars)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pressure(i32);

impl Pressure {
    pub fn with_microbars(µb: i32) -> Self {
        Self(µb)
    }

    pub fn microbars(&self) -> i32 {
        self.0
    }

    pub fn millibars(&self) -> i32 {
        self.0 / 1000
    }
}

impl Debug for Pressure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pressure({}.{:03} mb)", self.0 / 1000, self.0 % 1000)
    }
}

/// D-Value: the difference between the height of a particular pressure surface above sea level
/// and the height of that same isobaric surface from the U.S. Standard Atmosphere.
/// (stored in Meters)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DValue(i32);

impl DValue {
    pub fn with_meters(m: i32) -> Self {
        Self(m)
    }
    pub fn meters(&self) -> i32 {
        self.0
    }
}

impl Debug for DValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DValue({} m)", self.0)
    }
}

/// Angle
/// (stored in seconds)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Angle(u32);

impl Angle {
    pub fn with_degrees_minutes_seconds(d: u32, m: u32, s: u32) -> Self {
        Self(d * 60 * 60 + m * 60 + s)
    }

    pub fn degrees_minutes_seconds(&self) -> (u32, u32, u32) {
        (self.0 / (60 * 60), (self.0 % (60 * 60) / 60), (self.0 % 60))
    }
}

impl Debug for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (d, m, s) = self.degrees_minutes_seconds();
        write!(f, "{}º{}'{}\"", d, m, s)
    }
}

/// Geopotential Height.
/// (stored in Meters)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Altitude(u32);

impl Altitude {
    pub fn with_meters(meters: u32) -> Self {
        Self(meters)
    }
    pub fn meters(&self) -> u32 {
        self.0
    }
}

impl Debug for Altitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Altitude({} m)", self.0)
    }
}

/// Temperature
/// (stored in millikelvin)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Temperature(u32);

impl Temperature {
    pub fn with_millikelvin(mk: u32) -> Self {
        Self(mk)
    }

    pub fn with_millicelsius(mc: i32) -> Self {
        let mk = mc + 273150;
        if mk < 0 {
            panic!("Temperature less than absolute zero: {} mK", mk)
        }
        Self(mk as u32)
    }

    pub fn celsius(&self) -> u32 {
        (self.0 - 273150) / 1000
    }

    pub fn kelvin(&self) -> u32 {
        self.0 / 1000
    }
}

impl Debug for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature({}.{} K)", self.0 / 1000, self.0 % 1000)
    }
}

/// Speed
/// (stored in knots)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Speed(u32);

impl Speed {
    pub fn with_knots(kt: u32) -> Self {
        Self(kt)
    }
    pub fn knots(&self) -> u32 {
        self.0
    }
}

impl Debug for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Speed({} kt)", self.0)
    }
}

/// Rain rate
/// (stored in millimeters per hour)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RainRate(u32);

impl RainRate {
    pub fn with_mm_per_hr(mm_p_hr: u32) -> Self {
        Self(mm_p_hr)
    }
    pub fn mm_per_hr(&self) -> u32 {
        self.0
    }
}

impl Debug for RainRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RainRate({} mm/hr)", self.0)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Direction(Angle);

impl Direction {
    pub fn with_angle(angle: Angle) -> Direction {
        Self(angle)
    }
}

pub const NORTH: Direction = Direction(Angle(0));
pub const EAST: Direction = Direction(Angle(90 * 60 * 60));
pub const SOUTH: Direction = Direction(Angle(180 * 60 * 60));
pub const WEST: Direction = Direction(Angle(270 * 60 * 60));

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Wind {
    pub direction: Direction,
    pub speed: Speed,
}

impl Wind {
    pub fn with_direction_and_speed(direction: Direction, speed: Speed) -> Self {
        Self { direction, speed }
    }
}
