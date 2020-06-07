use web_sys::CanvasRenderingContext2d;
use super::engine::MOVEMENT_KEYS;

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

pub struct Region {
    /// Top left coordinate relative to origin
    /// 
    /// Maximum for height
    start: Coord,

    /// Bottom right coordinate relative to origin
    /// 
    /// Minimum for height
    end: Coord,
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

pub struct Entity {
    pub pos: Coord,
    pub velocity: Velocity,
    in_air: bool,
    hitbox: Region,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            pos: Coord::origin(),
            hitbox: Region::default_hitbox(),
            velocity: Velocity::new(),
            in_air: false,
        }
    }
    fn render(&self, ctx: &CanvasRenderingContext2d, _z_boundary: (f64, f64), canvas_center: (f64, f64), offset: (f64, f64)) {
        let size_mult = 1. / (self.hitbox.start.z / (self.pos.z + self.hitbox.start.z));
        let opacity = 100. * (self.hitbox.start.z / (self.pos.z + self.hitbox.start.z));
        // ctx.set_fill_style(&"white".into());
        // ctx.set_font(&"2em serif");
        // ctx.fill_text(&format!("size_mult: {:3.10} opacity: {:1.5}", size_mult, opacity), 10., 200.).unwrap();

        ctx.set_fill_style(&format!("rgba(0, 0, 255, {}%)", opacity).into());
        let x = canvas_center.0 + self.pos.x - offset.0 + (self.hitbox.start.x * size_mult);
        let y = canvas_center.1 + self.pos.y - offset.1 + (self.hitbox.start.y * size_mult);
        let w = canvas_center.0 + self.pos.x - offset.0 + (self.hitbox.end.x * size_mult) - x;
        let h = canvas_center.1 + self.pos.y - offset.1 + (self.hitbox.end.y * size_mult) - y;
        ctx.fill_rect(x, y, w, h);
    }

    pub fn tick(&mut self) {
        // Limit max velocity
        self.velocity.limit();

        // Simulate velocity falloff
        if self.velocity.to.x > 0. { self.velocity.to.x -= self.velocity.falloff.x }
        if self.velocity.to.x < 0. { self.velocity.to.x += self.velocity.falloff.x }
        
        if self.velocity.to.y > 0. { self.velocity.to.y -= self.velocity.falloff.y }
        if self.velocity.to.y < 0. { self.velocity.to.y += self.velocity.falloff.y }
        
        self.velocity.to.z -= self.velocity.falloff.z;
        
        // Apply movement
        self.pos = self.pos + self.velocity.to;

        if self.pos.z < 0. { self.pos.z = 0.; self.velocity.to.z = 0.; self.in_air = false; }
    }
}

pub struct Player {
    entity: Entity,
}

impl Player {
    fn new() -> Player {
        Player {
            entity: Entity::new(),
        }
    }
    fn render(&self, ctx: &CanvasRenderingContext2d, z_boundary: (f64, f64), canvas_center: (f64, f64), offset: (f64, f64)) {
        self.entity.render(ctx, z_boundary, canvas_center, offset);

        ctx.set_fill_style(&"white".into());
        ctx.set_font(&"2em serif");
        ctx.fill_text(&format!("pos: x: {:3.3} y: {:3.3} z: {:3.3} offset: {:3.3}, {:3.3}", self.entity.pos.x, self.entity.pos.y, self.entity.pos.z, offset.0, offset.1), 200., 100.).unwrap();
    }
    fn tick(&mut self) {
        unsafe {
            if MOVEMENT_KEYS.up    { self.entity.velocity.to.y -= self.entity.velocity.falloff.y * 2.; }
            if MOVEMENT_KEYS.down  { self.entity.velocity.to.y += self.entity.velocity.falloff.y * 2.; }
            if MOVEMENT_KEYS.left  { self.entity.velocity.to.x -= self.entity.velocity.falloff.x * 2.; }
            if MOVEMENT_KEYS.right { self.entity.velocity.to.x += self.entity.velocity.falloff.x * 2.; }
            if MOVEMENT_KEYS.jump && !self.entity.in_air { self.entity.velocity.to.z += self.entity.velocity.max.z; self.entity.in_air = true; }
        }
        self.entity.tick();
    }
}

pub struct World {
    pub player: Player,
    pub entities: Vec<Entity>,
    pub boundary: Region,
    /// Screen scroll amount relative to world origin (x, y)
    scroll: (f64, f64),
    /// Start scrolling screen when player hits given distance to canvas border (in percent)
    pub scroll_threshold: f64,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(),
            entities: Vec::new(),
            boundary: Region::default_boundary(),
            scroll: (0.0, 0.0),
            scroll_threshold: 0.85,
        }
    }
    fn scroll(&mut self, canvas_center: (f64, f64), canvas_size: (f64, f64)) {
        if self.player.entity.pos.x - self.scroll.0 + canvas_center.0 > canvas_size.0 * self.scroll_threshold {
            self.scroll.0 += self.player.entity.pos.x - self.scroll.0 + canvas_center.0 - canvas_size.0 * self.scroll_threshold;
        } else if self.player.entity.pos.x - self.scroll.0 + canvas_center.0 < canvas_size.0 * (1.0 - self.scroll_threshold) {
            self.scroll.0 += self.player.entity.pos.x - self.scroll.0 + canvas_center.0 - canvas_size.0 * (1.0 - self.scroll_threshold);
        }

        if self.player.entity.pos.y - self.scroll.1 + canvas_center.1 > canvas_size.1 * self.scroll_threshold {
            self.scroll.1 += self.player.entity.pos.y - self.scroll.1 + canvas_center.1 - canvas_size.1 * self.scroll_threshold;
        } else if self.player.entity.pos.y - self.scroll.1 + canvas_center.1 < canvas_size.1 * (1.0 - self.scroll_threshold) {
            self.scroll.1 += self.player.entity.pos.y - self.scroll.1 + canvas_center.1 - canvas_size.1 * (1.0 - self.scroll_threshold);
        }
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d, canvas_center: (f64, f64), canvas_size: (f64, f64)) {
        self.scroll(canvas_center, canvas_size);
        self.player.render(&ctx, (self.boundary.start.z, self.boundary.end.z), canvas_center, self.scroll);
        for entity in self.entities.iter_mut() {
            entity.render(&ctx, (self.boundary.start.z, self.boundary.end.z), canvas_center, self.scroll);
        }
    }
    pub fn tick(&mut self) {
        self.player.tick();
        for entity in self.entities.iter_mut() {
            entity.tick();
        }
    }
}