
/// List of all type of cube
#[derive(Clone)]
pub enum CubeType {
    Dirt,
    Stone,
}

/// Basic struct composing the world
#[derive(Clone)]
pub struct Cube {
    pub is_active: bool,
    cube_type: CubeType
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(CubeType::Dirt, true)
    }
}

impl Cube {
    /// cube's render size
    pub const CUBE_RENDER_SIZE: usize = 1;


    /// Create a new Cube
    pub fn new(cube_type: CubeType, is_active: bool) -> Cube {
        Cube {
            is_active,
            cube_type
        }
    }

}