use crate::measure::Angle;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LatitudeHemisphere {
    NORTH,
    SOUTH,
}

impl LatitudeHemisphere {
    pub fn short(&self) -> char {
        match self {
            Self::NORTH => 'N',
            Self::SOUTH => 'S',
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Latitude {
    pub angle: Angle,
    pub hemisphere: LatitudeHemisphere,
}

impl Debug for Latitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.angle, self.hemisphere.short())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LongitudeHemisphere {
    EAST,
    WEST,
}

impl LongitudeHemisphere {
    pub fn short(&self) -> char {
        match self {
            Self::EAST => 'E',
            Self::WEST => 'W',
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Longitude {
    pub angle: Angle,
    pub hemisphere: LongitudeHemisphere,
}

impl Debug for Longitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.angle, self.hemisphere.short())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Coordinate {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.latitude, self.longitude)
    }
}
