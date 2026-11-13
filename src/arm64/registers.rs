pub struct Registers {
    x: [u64; 31],
    sp: u64,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            x: [0; 31],
            sp: 0,
        }
    }

    pub fn read(&self, index: u8) -> u64 {
        match index {
            0..=30 => self.x[index as usize],
            31 => self.sp,
            _ => panic!("Invalid register index"),
        }
    }

    pub fn write(&mut self, index: u8, value: u64) {
        match index {
            0..=30 => self.x[index as usize] = value,
            31 => self.sp = value,
            _ => panic!("Invalid register index"),
        }
    }
}
