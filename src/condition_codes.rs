#[derive(Debug, Default)]
pub struct ConditionCodes {
    pub carry: bool,
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub aux_carry: bool,
}

impl ConditionCodes {
    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry
    }

    pub fn reset_carry(&mut self) {
        self.carry = false
    }

    pub fn set_zero(&mut self, val: u8) {
        self.zero = (val & 0xFF) == 0
    }

    pub fn set_sign(&mut self, val: u8) {
        self.sign = (val & 0x80) == 0x80
    }

    pub fn set_parity(&mut self, val: u8) {
        self.parity = val.count_ones() % 2 == 0
    }

    pub fn set_aux_carry(&mut self, aux_carry: bool) {
        self.aux_carry = aux_carry;
    }
}
