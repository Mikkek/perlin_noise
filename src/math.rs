#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// I don't know how much this is needed, but it can't hurt, i think
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}
