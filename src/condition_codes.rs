#[derive(Debug, Default)]
pub struct ConditionCodes {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

impl ConditionCodes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_all(&mut self) {
        self.zero = true;
        self.sign = true;
        self.parity = true;
        self.carry = true;
        self.aux_carry = true;
    }

    pub fn clear_all(&mut self) {
        self.zero = false;
        self.sign = false;
        self.parity = false;
        self.carry = false;
        self.aux_carry = false;
    }

    pub fn set_all_except_carry(&mut self) {
        self.set_all();
        self.carry = false;
    }

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

    pub fn set_aux_carry(&mut self, aux_carry: bool) {
        self.aux_carry = aux_carry
    }

    pub fn reset_aux_carry(&mut self) {
        self.aux_carry = false
    }
}
