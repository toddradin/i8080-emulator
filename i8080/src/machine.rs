pub trait MachineIO {
    fn machine_in(&mut self, port: u8) -> u8;

    fn machine_out(&mut self, port: u8, val: u8);
}
