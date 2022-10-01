pub enum Layer {
    GRID,
    PLAYER,
    UI
}

impl Layer {
    pub fn get_z(self) -> f32 {
        match self {
            Layer::GRID => 0.0,
            Layer::PLAYER => 1.0,
            Layer::UI => 10.0
        }
    }
}
