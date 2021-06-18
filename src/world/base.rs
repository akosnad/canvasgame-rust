#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Coordinate relative to middle of screen
#[derive(Debug, PartialEq)]
pub struct Coord {
    /// Horizontal offset
    /// 
    /// Positive goes right
    pub x: f64,

    /// Vertical offset
    /// 
    /// Positive goes down
    pub y: f64,

    /// Height offset (virually)
    /// 
    /// Ground level is zero, positive goes up. May indicate
    /// floor lever, or when an entity is jumping
    pub z: f64,
}

impl Coord {
    /// World origin
    /// 
    /// Middle of screen at initialization
    pub fn origin() -> Coord {
        Coord { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Default maximum `Velocity`
    pub fn default_max_vel() -> Coord {
        Coord { x: 3.5, y: 3.5, z: 1.5 }
    }

    /// Default `Velocity` falloff value
    pub fn default_vel_falloff() -> Coord {
        Coord { x: 0.25, y: 0.25, z: 0.05 }
    }
}

impl Copy for Coord { }
impl Clone for Coord {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(not(feature = "no_std"))]
impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl std::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(feature = "no_std")]
impl core::ops::Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(feature = "no_std")]
impl core::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


pub struct Region {
    /// Top left coordinate relative to origin
    /// 
    /// Maximum for height
    pub start: Coord,

    /// Bottom right coordinate relative to origin
    /// 
    /// Minimum for height
    pub end: Coord,
}

impl Region {
    /// Generic region for world boundary
    pub fn default_boundary() -> Region {
        Region {
            start: Coord { x: -1000., y: -1000., z: 1000. },
            end: Coord { x: 1000., y: 1000., z: 0. },
        }
    }
    /// Generic hitbox for entities
    pub fn default_hitbox() -> Region {
        Region {
            start: Coord { x: -16., y: -16., z: 16. },
            end: Coord {x: 16., y: 16., z: 0. },
        }
    }
}

/// Indicates a direction in movement
pub struct Velocity {
    /// Movement relative to current position
    pub to: Coord,
    /// Maximum velocity per axis
    pub max: Coord,
    /// Velocity falloff per tick, if not moving
    pub falloff: Coord,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            to: Coord::origin(),
            max: Coord::default_max_vel(),
            falloff: Coord::default_vel_falloff(),
        }
    }
    /// Limit `to` to `max`
    pub fn limit(&mut self) {
        if self.to.x > self.max.x { self.to.x = self.max.x }
        if self.to.x < -self.max.x { self.to.x = -self.max.x }

        if self.to.y > self.max.y { self.to.y = self.max.y }
        if self.to.y < -self.max.y { self.to.y = -self.max.y }

        // if self.to.z > self.max.z { self.to.z = self.max.z }
        // if self.to.z < -self.max.z { self.to.z = -self.max.z }
    }
}
