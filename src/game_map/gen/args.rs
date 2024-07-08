#[derive(Debug, Clone, Copy)]
pub struct GalaxyGenArgs {
    /// the number of planetary system in the galaxy
    pub size: u32,
    /// the density of the planetary system, in terms of 1/ly^2
    pub density: f32,
    /// the number of star remnant in the galaxy
    pub remnant: u32,
    /// the number of nebula in the galaxy
    pub nebula: u32,
}

impl Default for GalaxyGenArgs {
    fn default() -> Self {
        Self {
            size: 256,
            density: 0.25,
            remnant: 0,
            nebula: 0,
        }
    }
}
