pub mod bevy;
pub mod shipyard;
pub mod hecs;
pub mod specs;
pub mod legion;
pub mod legion_experimental;

/// Entities with velocity and position component.
/// Used to insert in staggered order
pub const N_POS_VEL_MODULUS: usize = 10;

/// Entities with position and velocity component
pub const N_POS_VEL: usize = 1000;

/// Entities with position component only
pub const N_POS: usize = 10000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}
