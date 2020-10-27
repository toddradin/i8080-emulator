#[derive(Default)]
pub struct ConditionCodes {
    pub z: bool,    // zero
    pub s: bool,    // sign
    pub p: bool,    // parity
    pub cy: bool,   // carry
    pub ac: bool,   // auxiliary carry
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


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_all() {
        let mut flags = ConditionCodes::new();
        flags.set_all();
        assert_eq!(flags.z, true);
        assert_eq!(flags.s, true);
        assert_eq!(flags.p, true);
        assert_eq!(flags.cy, true);
        assert_eq!(flags.ac, true);
    }

    #[test]
    fn test_clear_all() {
        let mut flags = ConditionCodes::new();
        flags.clear_all();
        assert_eq!(flags.z, false);
        assert_eq!(flags.s, false);
        assert_eq!(flags.p, false);
        assert_eq!(flags.cy, false);
        assert_eq!(flags.ac, false);
    }

    #[test]
    fn test_set_all_except_carry() {
        let mut flags = ConditionCodes::new();
        flags.set_all_except_carry();
        assert_eq!(flags.z, true);
        assert_eq!(flags.s, true);
        assert_eq!(flags.p, true);
        assert_eq!(flags.cy, false);
        assert_eq!(flags.ac, true);
    }

    #[test]
    fn test_set_zero() {
        let mut flags = ConditionCodes::new();
        flags.set_zero();
        assert_eq!(flags.z, true);
    }

    #[test]
    fn test_set_sign() {
        let mut flags = ConditionCodes::new();
        flags.set_sign();
        assert_eq!(flags.s, true);
    }

    #[test]
    fn test_set_parity() {
        let mut flags = ConditionCodes::new();
        flags.set_parity();
        assert_eq!(flags.p, true);
    }

    #[test]
    fn test_set_carry() {
        let mut flags = ConditionCodes::new();
        flags.set_carry();
        assert_eq!(flags.cy, true);
    }

    #[test]
    fn test_set_aux_carry() {
        let mut flags = ConditionCodes::new();
        flags.set_aux_carry();
        assert_eq!(flags.ac, true);
    }
}
