#[derive(Debug)]
pub struct ARange {
    #[allow(dead_code)]
    start: u32,
    pub end: u32,
}

impl ARange {
    pub fn new(end: u32) -> ARange {
        ARange { start: 0, end }
    }
    
    #[allow(dead_code)]
    pub fn get_private_field(&self) -> u32 {
        self.start
    }
}
