#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hl() {
        let mut reg = Registers::new();
        reg.h = 0x34;
        reg.l = 0x12;
        assert_eq!(reg.get_hl(), 0x3412);
    }
}
