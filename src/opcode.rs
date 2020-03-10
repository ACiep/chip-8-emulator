pub struct OPCode(pub u16);

impl OPCode {
    pub fn nnn(&self) -> u8 {
        (self.0 & 0x0FFF) as u8
    }

    pub fn nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    pub fn x(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    pub fn y(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }
}
