pub struct Screen([[bool; 32]; 64]);

impl Screen {
    pub fn new() -> Self {
        Self([[false; 32]; 64])
    }

    pub fn clear(&mut self) {
        self.0 = [[false; 32]; 64];
    }
}
