#[derive(Copy, Clone, Debug)]
pub enum Face {
    Right, // +X
    Left,  // -X
    Up,    // +Y
    Down,  // -Y
    Front, // +Z
    Back   // -Z
}