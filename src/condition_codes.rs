#[derive(Debug, Default)]
pub struct ConditionCodes {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

impl ConditionCodes {
    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry
    }

    pub fn reset_carry(&mut self) {
        self.carry = false
    }

    pub fn set_sign(&mut self, val: u8) {
        self.sign = (val & 0x80) == 0x80
    }

    pub fn set_zero(&mut self, val: u8) {
        self.zero = (val & 0xFF) == 0
    }

    pub fn set_parity(&mut self, val: u8) {
        self.parity = val.count_ones() % 2 == 0
    }
}

impl ConditionCodes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_zero(&mut self) {
        self.z = true;
    }

    pub fn set_sign(&mut self) {
        self.s = true;
    }

    pub fn set_parity(&mut self) {
        self.p = true;
    }

    pub fn set_carry(&mut self) {
        self.cy = true;
    }

    pub fn set_aux_carry(&mut self) {
        self.ac = true;
    }

    pub fn set_all(&mut self) {
        self.z = true;
        self.s = true;
        self.p = true;
        self.cy = true;
        self.ac = true;
    }

    pub fn clear_all(&mut self) {
        self.z = false;
        self.s = false;
        self.p = false;
        self.cy = false;
        self.ac = false;
    }

    pub fn set_all_except_carry(&mut self) {
        self.set_all();
        self.cy = false;
    }

    pub fn update_flags(&mut self, num: u16){
        self.z = (num & 0xFF) == 0;
        self.s = (num & 0x80) != 0;
        self.p = self.parity(num as u8);
        self.cy = (num & 0x0100) != 0;
        // TODO: aux carry 
    }

    fn parity(&self, check: u8) -> bool {
        let par = check.count_ones() % 2;
        // 1 for even, 0 for odd
        par == 0      
    }
}